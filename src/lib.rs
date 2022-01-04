use std::collections::HashMap;

use serde::de::DeserializeOwned;
use thiserror::Error;
use futures::future::try_join_all;

pub mod structs;

use structs::{
    contract::{ContractTicker, ContractTickerResult},
    positions::Position,
    transaction::Transaction,
};

#[derive(Error, Debug)]
pub enum FTXDerivativesError {
    #[error("error caught with reqwest")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },
    #[error("error caught on deserialization")]
    JSONError {
        #[from]
        source: serde_json::Error,
    },
}

pub struct FTXDerivatives {
    reqwest_client: reqwest::Client,
    api_key: String,
}

impl FTXDerivatives {
    pub fn new(api_key: &str) -> Self {
        FTXDerivatives {
            reqwest_client: reqwest::Client::new(),
            api_key: api_key.to_owned(),
        }
    }

    async fn get_list<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<structs::ListResult<T>, FTXDerivativesError> {
        let res = self
            .reqwest_client
            .get(url)
            .header("Authorization", format!("JWT {}", &self.api_key))
            .send()
            .await?
            .text()
            .await?;
        let json = serde_json::from_str(&res);
        if json.is_err() {
            println!("{}", res)
        }
        Ok(json?)
    }

    pub async fn get_positions(&self) -> Result<Vec<Position>, FTXDerivativesError> {
        const URL: &str = "https://api.ledgerx.com/trading/positions";

        Ok(self.get_list(URL).await?.data)
    }

    pub async fn get_transactions(&self) -> Result<Vec<Transaction>, FTXDerivativesError> {
        const URL: &str = "https://api.ledgerx.com/funds/transactions";

        Ok(self.get_list(URL).await?.data)
    }

    pub async fn get_contract_ticker(
        &self,
        contract_id: u64,
    ) -> Result<ContractTicker, FTXDerivativesError> {
        let url = format!(
            "https://api.ledgerx.com/trading/contracts/{}/ticker",
            contract_id
        );
        Ok(self
            .reqwest_client
            .get(url)
            .send()
            .await?
            .json::<ContractTickerResult>()
            .await?
            .data)
    }

    pub async fn get_contracts_ticker(
        &self,
        contract_ids: &[u64],
    ) -> Result<HashMap<u64, ContractTicker>, FTXDerivativesError> {
        let futs: Vec<_> = contract_ids.iter().map(|i| self.get_contract_ticker(*i)).collect();
        let res = try_join_all(futs).await?;
        Ok(contract_ids.iter().zip(res).map(|(x, y)| (*x, y)).collect())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use dotenv::dotenv;

    use super::*;

    #[tokio::test]
    async fn test_positions() {
        dotenv().ok();
        let client = FTXDerivatives::new(&env::var("API_KEY").unwrap());
        let pos = client.get_positions().await.unwrap();
        println!("{:#?}", pos);
    }

    #[tokio::test]
    async fn test_transactions() {
        dotenv().ok();
        let client = FTXDerivatives::new(&env::var("API_KEY").unwrap());
        let txn = client.get_transactions().await.unwrap();
        println!("{:#?}", txn);
    }

    #[tokio::test]
    async fn test_contract_ticker() {
        dotenv().ok();
        let client = FTXDerivatives::new(&env::var("API_KEY").unwrap());
        let ticker = client.get_contract_ticker(22227601).await.unwrap();
        println!("{:#?}", ticker);
    }

    #[tokio::test]
    async fn test_contracts_ticker() {
        dotenv().ok();
        let client = FTXDerivatives::new(&env::var("API_KEY").unwrap());
        let ticker = client.get_contracts_ticker(&[22227601, 22229249]).await.unwrap();
        println!("{:#?}", ticker);
    }
}

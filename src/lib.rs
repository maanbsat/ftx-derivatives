//! Library for FTX Derivatives (previously LedgerX) API access

use std::collections::HashMap;

use futures::future::try_join_all;
use rust_decimal::Decimal;
use serde::de::DeserializeOwned;
use thiserror::Error;

pub mod structs;

use structs::{
    contract::{Contract, ContractTicker, ContractTickerLastTrade, ContractTickerResult},
    positions::Position,
    trade::Trade,
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
    #[error("error caught in decimal processing")]
    DecimalError {
        #[from]
        source: rust_decimal::Error,
    },
    #[error("unknown currency")]
    UnknownCurrency { currency: String },
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
        // TODO: implement paging
        let res = self
            .reqwest_client
            .get(url)
            .query(&[("limit", 100)])
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
        let res: Vec<Position> = self.get_list(URL).await?.data;

        res.into_iter()
            .map(|p| {
                Ok(Position {
                    contract: convert_contract(p.contract)?,
                    ..p
                })
            })
            .collect()
    }

    pub async fn get_transactions(&self) -> Result<Vec<Transaction>, FTXDerivativesError> {
        const URL: &str = "https://api.ledgerx.com/funds/transactions";
        let res: Vec<Transaction> = self.get_list(URL).await?.data;

        res.into_iter().map(convert_transaction).collect()
    }

    pub async fn get_contract_ticker(
        &self,
        contract_id: u64,
    ) -> Result<ContractTicker, FTXDerivativesError> {
        let url = format!(
            "https://api.ledgerx.com/trading/contracts/{}/ticker",
            contract_id
        );
        let res = self
            .reqwest_client
            .get(url)
            .send()
            .await?
            .json::<ContractTickerResult>()
            .await?
            .data;
        convert_contract_ticker(res)
    }

    pub async fn get_contracts_ticker(
        &self,
        contract_ids: &[u64],
    ) -> Result<HashMap<u64, ContractTicker>, FTXDerivativesError> {
        let futs: Vec<_> = contract_ids
            .iter()
            .map(|i| self.get_contract_ticker(*i))
            .collect();
        let res = try_join_all(futs).await?;
        Ok(contract_ids.iter().zip(res).map(|(x, y)| (*x, y)).collect())
    }

    pub async fn get_trades(&self) -> Result<Vec<Trade>, FTXDerivativesError> {
        const URL: &str = "https://api.ledgerx.com/trading/trades";
        let res: Vec<Trade> = self.get_list(URL).await?.data;

        res.into_iter().map(convert_trade).collect()
    }

    pub async fn get_balances(&self) -> Result<HashMap<String, Decimal>, FTXDerivativesError> {
        let txn = self.get_transactions().await?;
        let mut balances = HashMap::new();

        for t in txn {
            if balances.contains_key(&t.asset) {
                *balances.get_mut(&t.asset).unwrap() += t.net_change;
            } else {
                balances.insert(t.asset.to_owned(), t.net_change);
            }
        }

        Ok(balances)
    }
}

fn get_num_decimals(currency: &str) -> Result<u32, FTXDerivativesError> {
    Ok(match currency {
        "USD" => 2,
        "CBTC" => 8,
        "ETH" => 9,
        _ => {
            return Err(FTXDerivativesError::UnknownCurrency {
                currency: currency.to_owned(),
            })
        }
    })
}

fn rescale_number(amount: Decimal, num_decimals: u32) -> Result<Decimal, FTXDerivativesError> {
    let mut res = amount;
    res.set_scale(num_decimals)?;
    Ok(res)
}

fn convert_contract(contract: Contract) -> Result<Contract, FTXDerivativesError> {
    match contract {
        Contract::Option {
            id,
            name,
            is_call,
            strike_price,
            min_increment,
            date_live,
            date_expires,
            date_exercise,
            open_interest,
            multiplier,
            label,
            active,
            underlying_asset,
            collateral_asset,
            option_type,
        } => Ok(Contract::Option {
            id,
            name,
            is_call,
            strike_price: rescale_number(strike_price, 2)?,
            min_increment,
            date_live,
            date_expires,
            date_exercise,
            open_interest,
            multiplier,
            label,
            active,
            underlying_asset,
            collateral_asset,
            option_type,
        }),
        x => Ok(x),
    }
}

fn convert_contract_ticker(ticker: ContractTicker) -> Result<ContractTicker, FTXDerivativesError> {
    let last_trade = match ticker.last_trade {
        Some(t) => Some(ContractTickerLastTrade {
            price: rescale_number(t.price, 2)?,
            ..t
        }),
        None => None,
    };
    Ok(ContractTicker {
        ask: rescale_number(ticker.ask, 2)?,
        bid: rescale_number(ticker.bid, 2)?,
        last_trade,
        ..ticker
    })
}

fn convert_transaction(transaction: Transaction) -> Result<Transaction, FTXDerivativesError> {
    fn rescale_opt(
        orig: Option<Decimal>,
        num_decimals: u32,
    ) -> Result<Option<Decimal>, FTXDerivativesError> {
        match orig {
            Some(o) => Ok(Some(rescale_number(o, num_decimals)?)),
            None => Ok(None),
        }
    }

    let num_decimals = get_num_decimals(&transaction.asset)?;

    Ok(Transaction {
        amount: rescale_number(transaction.amount, num_decimals)?,
        debit_pre_balance: rescale_opt(transaction.debit_pre_balance, num_decimals)?,
        debit_post_balance: rescale_opt(transaction.debit_post_balance, num_decimals)?,
        credit_pre_balance: rescale_opt(transaction.credit_pre_balance, num_decimals)?,
        credit_post_balance: rescale_opt(transaction.credit_post_balance, num_decimals)?,
        net_change: rescale_number(transaction.net_change, num_decimals)?,
        ..transaction
    })
}

fn convert_trade(trade: Trade) -> Result<Trade, FTXDerivativesError> {
    Ok(Trade {
        filled_price: rescale_number(trade.filled_price, 2)?,
        fee: rescale_number(trade.fee, 2)?,
        rebate: rescale_number(trade.rebate, 2)?,
        premium: rescale_number(trade.premium, 2)?,
        ..trade
    })
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
    async fn test_trades() {
        dotenv().ok();
        let client = FTXDerivatives::new(&env::var("API_KEY").unwrap());
        let trades = client.get_trades().await.unwrap();
        println!("{:#?}", trades);
    }

    #[tokio::test]
    async fn test_contract_ticker() {
        dotenv().ok();
        let client = FTXDerivatives::new(&env::var("API_KEY").unwrap());
        let ticker = client.get_contract_ticker(22212774).await.unwrap();
        println!("{:#?}", ticker);
    }

    #[tokio::test]
    async fn test_contracts_ticker() {
        dotenv().ok();
        let client = FTXDerivatives::new(&env::var("API_KEY").unwrap());
        let ticker = client
            .get_contracts_ticker(&[22212774, 22210648])
            .await
            .unwrap();
        println!("{:#?}", ticker);
    }

    #[tokio::test]
    async fn test_balances() {
        dotenv().ok();
        let client = FTXDerivatives::new(&env::var("API_KEY").unwrap());
        let balances = client.get_balances().await.unwrap();
        println!("{:#?}", balances);
    }
}

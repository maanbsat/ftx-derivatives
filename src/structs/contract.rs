use serde::Deserialize;

use super::DateTime;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DerivativeType {
    OptionsContract,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Deserialize, Debug)]
pub struct Contract {
    pub id: u64,
    pub name: Option<String>,
    pub is_call: bool,
    pub strike_price: u64,
    pub min_increment: u32,
    pub date_live: DateTime,
    pub date_expires: DateTime,
    pub date_exercise: DateTime,
    pub derivative_type: DerivativeType,
    pub open_interest: u64,
    pub multiplier: u32,
    pub label: String,
    pub active: bool,
    pub is_next_day: bool,
    pub underlying_asset: String,
    pub collateral_asset: String,
    #[serde(rename = "type")]
    pub option_type: OptionType,
}

#[derive(Deserialize, Debug)]
pub struct ContractTickerResult {
    pub data: ContractTicker,
}

#[derive(Deserialize, Debug)]
pub struct ContractTicker {
    pub ask: u32,
    pub bid: u32,
    pub volume_24h: u32,
    pub last_trade: Option<ContractTickerLastTrade>,
    pub time: DateTime,
}

#[derive(Deserialize, Debug)]
pub struct ContractTickerLastTrade {
    pub id: u64,
    pub price: u32,
    pub size: u32,
    pub time: DateTime,
}

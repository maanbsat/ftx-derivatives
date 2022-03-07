use rust_decimal::Decimal;
use serde::Deserialize;

use super::DateTime;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "derivative_type")]
pub enum Contract {
    #[serde(rename = "day_ahead_swap")]
    DayAheadSwap {
        id: u64,
        name: Option<String>,
        min_increment: u32,
        date_live: DateTime,
        date_expires: DateTime,
        date_exercise: DateTime,
        open_interest: u64,
        multiplier: Decimal,
        label: String,
        active: bool,
        is_next_day: bool,
        underlying_asset: String,
        collateral_asset: String,
    },
    #[serde(rename = "options_contract")]
    Option {
        id: u64,
        name: Option<String>,
        is_call: bool,
        strike_price: Decimal,
        min_increment: u32,
        date_live: DateTime,
        date_expires: DateTime,
        date_exercise: DateTime,
        open_interest: u64,
        multiplier: Decimal,
        label: String,
        active: bool,
        underlying_asset: String,
        collateral_asset: String,
        #[serde(rename = "type")]
        option_type: OptionType,
    },
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContractTickerResult {
    pub data: ContractTicker,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContractTicker {
    pub ask: Decimal,
    pub bid: Decimal,
    pub volume_24h: u32,
    pub last_trade: Option<ContractTickerLastTrade>,
    pub time: DateTime,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContractTickerLastTrade {
    pub id: u64,
    pub price: Decimal,
    pub size: u32,
    pub time: DateTime,
}

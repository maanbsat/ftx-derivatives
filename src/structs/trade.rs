use rust_decimal::Decimal;
use serde::Deserialize;

use super::DateTime;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    CustomerLimitOrder,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TradeSide {
    Bid,
    Ask,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Trade {
    pub id: u64,
    pub contract_id: String,
    pub contract_label: String,
    pub filled_price: Decimal,
    pub filled_size: u32,
    pub fee: Decimal,
    pub rebate: Decimal,
    pub premium: Decimal,
    pub created: DateTime,
    pub order_type: OrderType,
    pub order_id: String,
    pub state: Option<String>,
    pub status_type: String,
    pub side: TradeSide,
    pub execution_time: DateTime,
}

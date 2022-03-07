use serde::Deserialize;
use rust_decimal::Decimal;

use super::DateTime;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")] 
pub enum TransactionType {
    FeeTransaction,
    PositionLockTransaction,
    ReleasePositionLockTransaction,
    PremiumTransaction,
    DepositTransaction,
    WithdrawalTransaction,
    WithdrawalLockTransaction,
    SettlementLockTransaction,
    ReleaseSettlementLockTransaction,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")] 
pub enum TransactionState {
    Pending,
    Cached,
    Executed,
    Failed,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: u64,
    pub created: DateTime,
    pub last_updated: DateTime,
    #[serde(rename = "poly")]
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub debit_account_field_name: String,
    pub credit_account_field_name: String,
    pub settlement_id: Option<u64>,
    pub state: TransactionState,
    pub deposit_notice_id: Option<u64>,
    pub trade_id: Option<u64>,
    pub group_id: Option<String>,
    pub asset: String,
    pub debit_pre_balance: Option<Decimal>,
    pub debit_post_balance: Option<Decimal>,
    pub credit_pre_balance: Option<Decimal>,
    pub credit_post_balance: Option<Decimal>,
    // pub debit_account_mpid: Option<String>,
    pub debit_participant_name: Option<String>,
    // pub credit_account_mpid: Option<String>,
    pub credit_participant_name: Option<String>,
    pub net_change: Decimal,
}

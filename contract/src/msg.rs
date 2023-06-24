use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Binary,
    Uint128
};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Withdraw(WithdrawMsg),
    Deposit(Cw20ReceiveMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WithdrawMsg {
    pub cw20_address: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    Deposit(DepositMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DepositMsg {
    pub cw20_address: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryMsg {
    DepositsList {},
}

#[cw_serde]
pub struct GetTreeResponse {
    pub count: Binary,
}
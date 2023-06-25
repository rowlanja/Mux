use crate::msg::{Cw20HookMsg, GetDepositsResp, DepositMsg, ExecuteMsg, InstantiateMsg, QueryMsg, WithdrawMsg};
use crate::state::{TREE, DEPOSITS, Deposit, Config, CONFIG};
use crate::error::ContractError;

use cosmwasm_std::StdError;
use rs_merkle::{MerkleTree, algorithms::Sha256, Hasher};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use cosmwasm_std::{
    from_binary, 
    to_binary, 
    Binary, 
    Deps, 
    DepsMut, 
    BankMsg,
    Env, 
    MessageInfo,
    Response, 
    StdResult, 
    CosmosMsg, 
    Coin,
    Addr,
    WasmMsg, 
    entry_point,
    Empty
};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let leaves = [
        Sha256::hash("bemo".as_bytes()),
    ];
    let data = Binary(MerkleTree::<Sha256>::from_leaves(&leaves).root().unwrap().to_vec());
    let config = Config {
        arbiter: deps.api.addr_validate(&msg.arbiter)?,
        source: info.sender,
    };

    CONFIG.save(deps.storage, &config)?;
    // TREE.save(deps.storage, &data)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Withdraw {} => withdraw_cw20(deps, info),
        ExecuteMsg::Deposit { quantity } => deposit_cw20(deps, env, info, quantity),
    }
}
fn withdraw_cw20(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // Get the params from WithdrawMsg
    // let cw20_address = msg.cw20_address;
    // let to_sent = msg.amount;

    // // Validations
    // let cw20_address = deps.api.addr_validate(cw20_address.as_str())?;
    // // check if the "to_sent" amount is greater than "max_cap" of "cw20_address" token.
    // if to_sent.is_zero() {
    //     // return StdError::GenericErr {
    //     //     msg: "Invalid zero amount".to_string(),
    //     // };
    //     return Ok(Response::default());
    // }

    // // Handle the real "withdraw"
    // let recipient = deps.api.addr_validate(info.sender.as_str())?;
    // let msgs: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
    //     contract_addr: cw20_address.to_string(),
    //     msg: to_binary(&Cw20ExecuteMsg::Transfer {
    //         recipient: recipient.to_string(),
    //         amount: to_sent,
    //     })?,
    //     funds: vec![],
    // })];

    Ok(Response::default())
}


// add amount and hash of secret to merkle tree on deposit
// then add testing
fn deposit_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    quantity: Option<Vec<Coin>>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.arbiter {
        return Err(ContractError::Unauthorized {});
    }
    let amount = if let Some(quantity) = quantity {
        quantity
    } else {
        // release everything
        // Querier guarantees to return up-to-date data, including funds sent in this handle message
        // https://github.com/CosmWasm/wasmd/blob/master/x/wasm/internal/keeper/keeper.go#L185-L192
        deps.querier.query_all_balances(&env.contract.address)?
    };

    let sender = info.sender;
    let sent_amount = &amount;
    
    let mut deposits = DEPOSITS.load(deps.storage)?;
    deposits.push(
        Deposit {
            addr: sender.to_string(),
            amount: sent_amount.clone()
        }
    );
    DEPOSITS.save(deps.storage,  &deposits)?;
    Ok(send_tokens(config.arbiter, amount, "approve"))
}

// this is a helper to move the tokens, so the business logic is easy to read
fn send_tokens(to_address: Addr, amount: Vec<Coin>, action: &str) -> Response {
    Response::new()
        .add_message(BankMsg::Send {
            to_address: to_address.clone().into(),
            amount,
        })
        .add_attribute("action", action)
        .add_attribute("to", to_address)
}


pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        DepositsList {} => to_binary(&query::deposits(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn deposits(deps: Deps) -> StdResult<GetDepositsResp> {
        let state = DEPOSITS.load(deps.storage)?;
        Ok(GetDepositsResp { count: state })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, CosmosMsg, Timestamp};
    use cw_utils::Expiration;

    use super::*;

    #[test]
    fn deposit_test() {
        let mut deps = mock_dependencies();

        // initialize the store
        let init_amount = coins(1000, "earth");
        let mut env = mock_env();
        env.block.height = 876;
        env.block.time = Timestamp::from_seconds(0);
        let info = mock_info("creator", &init_amount);
        let contract_addr = env.clone().contract.address;
        let init_res = instantiate(deps.as_mut(), env, info, InstantiateMsg{ arbiter : String::from("creator")}).unwrap();
        assert_eq!(0, init_res.messages.len());
    }
}


use crate::msg::{Cw20HookMsg, GetDepositsResp, DepositMsg, ExecuteMsg, InstantiateMsg, QueryMsg, WithdrawMsg};
use crate::state::{TREE, DEPOSITS,Deposit};
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
    Env, 
    MessageInfo,
    Response, 
    StdResult, 
    CosmosMsg, 
    WasmMsg, 
    entry_point,
    Empty
};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let leaves = [
        Sha256::hash("bemo".as_bytes()),
    ];
    let data = Binary(MerkleTree::<Sha256>::from_leaves(&leaves).root().unwrap().to_vec());

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
        ExecuteMsg::Withdraw(msg) => withdraw_cw20(deps, info, msg),
        ExecuteMsg::Deposit(msg) => deposit_cw20(deps, info, msg),
    }
}
fn withdraw_cw20(
    deps: DepsMut,
    info: MessageInfo,
    msg: WithdrawMsg,
) -> Result<Response, ContractError> {
    // Get the params from WithdrawMsg
    let cw20_address = msg.cw20_address;
    let to_sent = msg.amount;

    // Validations
    let cw20_address = deps.api.addr_validate(cw20_address.as_str())?;
    // check if the "to_sent" amount is greater than "max_cap" of "cw20_address" token.
    if to_sent.is_zero() {
        return Err(ContractError::Std(StdError::GenericErr {
            msg: "Invalid zero amount".to_string(),
        }));
    }

    // Handle the real "withdraw"
    let recipient = deps.api.addr_validate(info.sender.as_str())?;
    let msgs: Vec<CosmosMsg> = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: cw20_address.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: recipient.to_string(),
            amount: to_sent,
        })?,
        funds: vec![],
    })];

    Ok(Response::default().add_messages(msgs))
}


// add amount and hash of secret to merkle tree on deposit
// then add testing
fn deposit_cw20(
    deps: DepsMut,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let token_contract = info.sender;
    let sent_amount = cw20_msg.amount;

    // Deseralize the message for the params
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Deposit(msg)) => {
            let DepositMsg {
                cw20_address,
                amount,
            } = msg;
            // Validations
            if sent_amount != amount {
                return Err(ContractError::Std(StdError::GenericErr {
                    msg: "Invalid amount".to_string(),
                }));
            }
            if token_contract != deps.api.addr_validate(cw20_address.as_str())? {
                return Err(ContractError::Std(StdError::GenericErr {
                    msg: "Invalid amount".to_string(),
                }));
            }

            // Handle the real "deposit".
            
            let mut deposits = DEPOSITS.load(deps.storage)?;
            deposits.push(
                Deposit {
                    addr: cw20_address,
                    amount: amount
                }
            );
            DEPOSITS.save(deps.storage,  &deposits)?;
            Ok(Response::default())
        }
        Err(_) => Err(ContractError::Unauthorized {}),
    }
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
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};
    use cosmwasm_std::from_binary;
    use cosmwasm_std::Uint128;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw20::Cw20ReceiveMsg;

    use super::*;

    #[test]
    fn deposit_test() {
        // Create virtual chain
        let mut app = App::default();

        // Create contract representation
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &Empty {},
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: Result<Response, ContractError> = app
            .wrap()
            .query_wasm_smart(addr, &ExecuteMsg::Deposit(
                Cw20ReceiveMsg {
                    sender: Addr::unchecked("owner").to_string(),
                    amount: Uint128::new(0),
                    msg: Binary("hello".as_bytes().to_vec()),
                })
            )
            .unwrap(); 
        
        let results: GetDepositsResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::DepositsList {})
            .unwrap();
        print!("{:?}", results) 
    }
}


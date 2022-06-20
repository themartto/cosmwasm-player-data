#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:my-first-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Add { player, data } => add_p_data(deps, player, data),
        ExecuteMsg::Get { player, data } => update_p_data(deps,player, data),
    }
}

pub fn add_p_data(deps: DepsMut, p: Addr, d: Addr) -> Result<Response, ContractError> {
    STATE.save(deps.storage, p, &d)?;

    Ok(Response::new().add_attribute("method", "add_p_data"))
}

pub fn update_p_data(_deps: DepsMut, _p: Addr, _d: Addr) -> Result<Response, ContractError> {
    // STATE.update(deps.storage, p, |mut state| {
    //         state = Option::from(d)
    //     })?;
    Ok(Response::new().add_attribute("method", "update_p_data"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPData {p} => to_binary(&query_count(deps, p)?),
    }
}

fn query_count(deps: Deps, p: Addr) -> StdResult<CountResponse> {
    let p_data = STATE.load(deps.storage, p)?;

    Ok(CountResponse { pdata: p_data })
}

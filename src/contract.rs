use cosmwasm_std::{
    entry_point,  Binary, Deps, DepsMut, Env, 
    MessageInfo, Response,StdResult,StdError
};


use crate::msg::{InstantiateMsg,PropertyMsg,PropertyExecuteMsg};

use crate::instatiate;
use crate::query;
use crate::execute;




#[entry_point]
pub fn instantiate_proprty_smartcontract(
    deps:DepsMut,
    env:Env,
    info:MessageInfo,
    msg:InstantiateMsg
)-> StdResult<Response>{

     instatiate::instatiate_property(deps, env, info, msg)

}


#[entry_point]
pub fn query_property_smartcontract(

    deps:Deps,
    env : Env,
    msg : PropertyMsg,
) -> StdResult<Binary> {
    
    query::query_propert(deps, env, msg)

}

#[entry_point]
pub fn exceute_property_smartcontract(
    deps:DepsMut,
    env:Env,
    info:MessageInfo,
    msg: PropertyExecuteMsg
)-> Result<Response,StdError>{
   
   execute::execute_property(deps, info, msg, env)

}


use cosmwasm_std::{
    DepsMut,Env,MessageInfo,StdResult,Response,Addr
};

use crate::msg::InstantiateMsg;

use crate::state::{Property,PROPERTIES};



pub fn instatiate_property(
    deps : DepsMut,
    _env:Env,
    _info:MessageInfo,
    _msg : InstantiateMsg
)-> StdResult<Response>{

    let initial_property = Property{
        address : Addr::unchecked(""),
        description: "".to_owned(),
        seller_contact : 0,
        seller_name : Addr::unchecked(""),
        price : 0
    };

    let mut list_of_property = Vec::new();
    list_of_property.push(initial_property);
    

    PROPERTIES.save(deps.storage,&list_of_property)?;
   

    Ok(Response::default())

}
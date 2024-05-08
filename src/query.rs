

use cosmwasm_std :: {
    to_json_binary, Binary, Deps, Env,StdResult
};

use crate::msg::PropertyMsg;


pub fn query_propert(
    deps : Deps,
    _env : Env,
    msg : PropertyMsg
)-> StdResult<Binary>{

    use PropertyMsg::*;

     match  msg{

       GetAllProperty { } => to_json_binary(&impl_query::get_list_of_property(deps)?)
     }
}



mod impl_query{
    use cosmwasm_std::{Deps, StdResult};

    use crate::response::ListOfProperty;
    use crate::state::PROPERTIES;


   pub fn get_list_of_property(deps:Deps)-> StdResult<ListOfProperty>{

        let properties = PROPERTIES.load(deps.storage)?;

        let resp = ListOfProperty{ list_of_property : properties};

        Ok(resp)
    }
}

  
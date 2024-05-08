
use cosmwasm_std::{
    DepsMut,Env,MessageInfo,StdResult,Response
};

use crate::msg::PropertyExecuteMsg;


pub fn execute_property(
  deps:DepsMut,
  info:MessageInfo,
  msg:PropertyExecuteMsg,
  _env : Env
)-> StdResult<Response>{


      use PropertyExecuteMsg::*;

      match msg{

        AddProperty{
          address,
          description,
          seller_contact,
          seller_name,
          price
        }=> impl_execute_property::
        add_property(
          deps, 
          info,
           address, description, 
           seller_contact, seller_name, price),

           RemoveProperty {  } => impl_execute_property::
             remove_property(deps, info),      
      }

}


mod impl_execute_property{

   use cosmwasm_std::{DepsMut, Event, MessageInfo, Response, StdError, StdResult};

    use crate::state::{Property, PROPERTIES};
   
   
    pub fn add_property(
      deps:DepsMut,
      _info:MessageInfo,
      address: String,
     description : String,
     seller_contact: u64,
      seller_name: String,
      price: u64
    )-> Result<Response,StdError>{


      let seller_addr = deps.api
         .addr_validate(&seller_name).unwrap();

        let addre_string_into_addr = deps.api
                      .addr_validate(&address).unwrap();

        let mut list_of_prooperty = PROPERTIES.load(deps.storage)?;    

        let new_property = Property{
          address:addre_string_into_addr,
           seller_contact,
           seller_name : seller_addr,
           price,
           description
        };

        list_of_prooperty.push(new_property.clone());  

        PROPERTIES.save(deps.storage, &list_of_prooperty); 


        let event : Event = Event::new("add_new_property")
        .add_attribute("new_property_address", new_property.address);
        //.add_attribute("NoOfPropertyHave", list_of_prooperty.len().to_string());  

        let response = Response::new()
              .add_event(event);
    
       Ok(response)

    }


    pub fn remove_property(
      deps : DepsMut,
      info : MessageInfo
    )->StdResult<Response>{

         let remove_property = info.sender.clone();

         PROPERTIES.update(deps.storage, move|property|-> StdResult<_>{

             let properties = property
                  .into_iter()
                  .filter(|property| property.seller_name != info.sender)
                  .collect();

                Ok(properties)
         })?;

         let event : Event = Event::new("remove_property")
               .add_attribute("property_addr", remove_property);

              let response = Response::new()
                   .add_event(event);

                  Ok(response)

    }

}
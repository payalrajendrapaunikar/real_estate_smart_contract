

use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct Property{

   pub address: Addr,
   pub  description : String,
   pub seller_contact: u64,
   pub seller_name: Addr,
   pub price: u64
}



//defined storage entity of smart contract
pub const PROPERTIES : Item<Vec<Property>> = Item::new("properties");



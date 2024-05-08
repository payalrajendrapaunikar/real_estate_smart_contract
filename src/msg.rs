
use serde::{Serialize,Deserialize};




#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
pub struct InstantiateMsg{
      
}


#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
pub enum PropertyMsg{
      GetAllProperty{}
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub enum PropertyExecuteMsg{

      AddProperty{
          address: String,
          description : String,
          seller_contact: u64,
          seller_name: String,
          price: u64
      },

      RemoveProperty{}

      
}
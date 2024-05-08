

use crate::state::Property;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,PartialEq,Clone)]
pub struct ListOfProperty{
    pub list_of_property : Vec<Property>,
}
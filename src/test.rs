




#[cfg(test)]
mod tests{


    use cosmwasm_std::Addr;
    use cw_multi_test::{App,ContractWrapper,Executor};

    use crate::{contract::exceute_property_smartcontract, instatiate::instatiate_property, query::query_propert};

    
    use crate::msg::{InstantiateMsg,PropertyMsg,PropertyExecuteMsg};

    use crate::response::ListOfProperty;

    use crate::state::Property;

    

    #[test]
    fn test_instatiate_smart_by_get_list_of_property_intial_data(){

       // println!("inside the tst case");

        let mut app = App::default();

        let code = ContractWrapper::new(
            exceute_property_smartcontract, 
            instatiate_property,
             query_propert
            );

            let code_id = app.store_code(Box::new(code));

           //println!("code id is : {}",code_id);

            let addr = app 
               .instantiate_contract(
                code_id,
                 Addr::unchecked("owner"), 
                 &InstantiateMsg{ }, 
                 &[], 
                 "PropertySmartContract", 
                 None)
                 .unwrap();


                 

                // println!("address is : {}",addr.to_string());


                 let resp : ListOfProperty = app
                 .wrap()
                 .query_wasm_smart(addr, &PropertyMsg::GetAllProperty {  })
                 .unwrap();


                assert_eq!(
                    resp,
                    ListOfProperty{
                        list_of_property: vec![
                          Property  {
                                address : Addr::unchecked(""),
                                description: "".to_owned(),
                                seller_contact : 0,
                                seller_name : Addr::unchecked(""),
                             price : 0
                            }
                        ]
                    }
                )
        
    }


    //here first I add property then i get add property through query!
    #[test]
    pub fn test_add_new_property(){

        let mut app = App::default();

        let code = ContractWrapper::new(
            exceute_property_smartcontract, 
            instatiate_property,
             query_propert
            );

            let code_id = app.store_code(Box::new(code));

           //println!("code id is : {}",code_id);

            let addr = app 
               .instantiate_contract(
                code_id,
                 Addr::unchecked("owner"), 
                 &InstantiateMsg{ }, 
                 &[], 
                 "PropertySmartContract", 
                 None)
                 .unwrap();


                let excute_add_new_property = app
                  .execute_contract(
                    Addr::unchecked("plot no 45 bajaj nagar nagpur"),
                     addr.clone(), 
                     &PropertyExecuteMsg::AddProperty { 
                        address: "plot no 45 bajaj nagar nagpur".to_owned(), 
                        description: "1024 sq ".to_owned(),
                        seller_contact: 12345678,
                       seller_name: "payal".to_owned(), 
                       price: 1200000,
                    },
                     &[]
                ).unwrap();


                let wasm = excute_add_new_property
                    .events
                    .into_iter()
                    .find(|ev|ev.ty == "wasm-add_new_property")
                    .expect("event type is not found");


                let attribute_value = wasm
                            .attributes
                            .iter()
                            .find(|attr|attr.key == "new_property_address")
                            .expect("attribute not found");

                        assert_eq!(
                           attribute_value.value,
                           "plot no 45 bajaj nagar nagpur" 
                        );


                        let resp : ListOfProperty = app
                        .wrap()
                        .query_wasm_smart(addr, &PropertyMsg::GetAllProperty {  })
                        .unwrap();


                    
       
       
                       assert_eq!(
                           resp,
                           ListOfProperty{
                               list_of_property: vec![
                                 Property  {
                                       address : Addr::unchecked(""),
                                       description: "".to_owned(),
                                       seller_contact : 0,
                                       seller_name : Addr::unchecked(""),
                                    price : 0
                                   },
                                   Property  {
                                    address : Addr::unchecked("plot no 45 bajaj nagar nagpur"),
                                    description: "1024 sq ".to_owned(),
                                    seller_contact : 12345678,
                                    seller_name : Addr::unchecked("payal"),
                                 price : 1200000,
                                }

                               ]
                           }
                       )

                        

    }

    #[test]
    pub fn test_remove_property(){


        let mut app = App::default();

        let code = ContractWrapper::new(
            exceute_property_smartcontract, 
            instatiate_property,
             query_propert
            );

            let code_id = app.store_code(Box::new(code));

           //println!("code id is : {}",code_id);

            let addr = app 
               .instantiate_contract(
                code_id,
                 Addr::unchecked("owner"), 
                 &InstantiateMsg{ }, 
                 &[], 
                 "PropertySmartContract", 
                 None)
                 .unwrap();


                let excute_add_new_property = app
                  .execute_contract(
                    Addr::unchecked("plot no 45 bajaj nagar nagpur"),
                     addr.clone(), 
                     &PropertyExecuteMsg::AddProperty { 
                        address: "plot no 45 bajaj nagar nagpur".to_owned(), 
                        description: "1024 sq ".to_owned(),
                        seller_contact: 12345678,
                       seller_name: "payal".to_owned(), 
                       price: 1200000,
                    },
                     &[]
                ).unwrap();


                let wasm = excute_add_new_property
                    .events
                    .into_iter()
                    .find(|ev|ev.ty == "wasm-add_new_property")
                    .expect("event type is not found");


                let attribute_value = wasm
                            .attributes
                            .iter()
                            .find(|attr|attr.key == "new_property_address")
                            .expect("attribute not found");

                        assert_eq!(
                           attribute_value.value,
                           "plot no 45 bajaj nagar nagpur" 
                        );


                        let resp : ListOfProperty = app
                        .wrap()
                        .query_wasm_smart(addr.clone(), &PropertyMsg::GetAllProperty {  })
                        .unwrap();


                    
       
       
                       assert_eq!(
                           resp,
                           ListOfProperty{
                               list_of_property: vec![
                                 Property  {
                                       address : Addr::unchecked(""),
                                       description: "".to_owned(),
                                       seller_contact : 0,
                                       seller_name : Addr::unchecked(""),
                                    price : 0
                                   },
                                   Property  {
                                    address : Addr::unchecked("plot no 45 bajaj nagar nagpur"),
                                    description: "1024 sq ".to_owned(),
                                    seller_contact : 12345678,
                                    seller_name : Addr::unchecked("payal"),
                                 price : 1200000,
                                }

                               ]
                           }
                       );

              let excute_remove_property = app
              .execute_contract(
                Addr::unchecked("plot no 45 bajaj nagar nagpur"), 
                addr.clone(),
                 &PropertyExecuteMsg::RemoveProperty {  },
                  &[]
                ).unwrap();

                let wasm_event = excute_remove_property
                     .events
                     .into_iter()
                     .find(|ev|ev.ty == "wasm-remove_property")
                     .expect("event type not found");

                    let remove_property_attributes = wasm_event
                                   .attributes
                                   .iter()
                                    .find(|attr|attr.key == "property_addr")
                                    .expect("attribute not found in remove property");

                                assert_eq!(
                                    remove_property_attributes.value,
                                    "plot no 45 bajaj nagar nagpur"
                                );

    }

}
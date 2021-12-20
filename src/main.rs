use std::collections::BTreeMap;

// mod guess_game;
// mod new_variable;
// mod statement_expression;
// mod match_statement;
// mod enumeration;
// mod unions;
// mod array_tupple;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct MyStruct {
    bool_value: bool,
    str_value: String,
    uint_value: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct MyStruct2 {
    btree_storage: BTreeMap<String, String>,
}

fn main() {
    // let my_struct = MyStruct {
    //     bool_value: true,
    //     str_value: String::from("Hello, world"),
    //     uint_value: 100,
    // };
    // let encoded_my_struct = my_struct.try_to_vec().unwrap();
    // let decoded_my_struct = MyStruct::try_from_slice(&encoded_my_struct).unwrap();
    // print!("{:?}", decoded_my_struct);
    let mut btree_map: BTreeMap<String, String> = BTreeMap::new();
    btree_map.insert(String::from("Hello"), String::from("World"));
    btree_map.insert(String::from("Hello2"), String::from("Value2"));
    let my_struct_2 = MyStruct2 {
        btree_storage: btree_map,
    };
    let encoded_my_struct_2 = my_struct_2.try_to_vec().unwrap();
    let decoded_my_struct_2 = MyStruct2::try_from_slice(&encoded_my_struct_2).unwrap();
    print!("{:?}", decoded_my_struct_2);
}

use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::BTreeMap;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct MyStruct {
    bool_value: bool,
    str_value: String,
    uint_value: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct MyStruct2 {
    btree_storage: BTreeMap<String, MyStruct>,
}
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct MyStruct3 {
    vector_storage: Vec<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct StructWithOptionalField {
    optional_number: Option<[u8; 32]>, // Emulate public key
}

pub fn test_2() {
    let struct_option = StructWithOptionalField {
        optional_number: Option::Some([0; 32]),
    };
    let encoded_struct_option = struct_option.try_to_vec().unwrap();
    println!("{:?}", encoded_struct_option);

    // 1000
    // let arr: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 232];
    // let number: u128 = u128::from_be_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 232]);
    // print!("{}", number);
}

pub fn test_1() {
    let mut my_vector_storage: Vec<String> = Vec::new();
    my_vector_storage.push(String::from("Hello"));
    my_vector_storage.push(String::from("World"));
    let my_struct_3 = MyStruct3 {
        vector_storage: my_vector_storage,
    };
    let encoded_my_struct_3 = my_struct_3.try_to_vec().unwrap();
    print!("{:?}", encoded_my_struct_3.len());
    let decoded_my_struct_3 = MyStruct3::try_from_slice(&encoded_my_struct_3).unwrap();
    print!("{:?}", decoded_my_struct_3);
    let my_struct = MyStruct {
        bool_value: true,
        str_value: String::from("Hello, world"),
        uint_value: 100,
    };
    let encoded_my_struct = my_struct.try_to_vec().unwrap();
    let decoded_my_struct = MyStruct::try_from_slice(&encoded_my_struct).unwrap();
    print!("{:?}", decoded_my_struct);
    let mut btree_map: BTreeMap<String, MyStruct> = BTreeMap::new();
    btree_map.insert(
        String::from("Hello"),
        MyStruct {
            bool_value: true,
            str_value: String::from("World"),
            uint_value: 100,
        },
    );
    btree_map.insert(
        String::from("Hello2"),
        MyStruct {
            bool_value: false,
            str_value: String::from("World2"),
            uint_value: 50,
        },
    );
    let my_struct_2 = MyStruct2 {
        btree_storage: btree_map,
    };
    let encoded_my_struct_2 = my_struct_2.try_to_vec().unwrap();
    let decoded_my_struct_2 = MyStruct2::try_from_slice(&encoded_my_struct_2).unwrap();
    print!("{:?}", decoded_my_struct_2);
}

use std::mem;

pub fn test() {
    let mut num_arr: [u32; 5] = [1, 2, 3, 4, 5];
    let num_tupple: (u32, char, bool) = (100, 'S', true);

    println!("Num_arr length is {}", num_arr.len());
    for n in 0..=4 {
        println!("Number at num_arr index {} is {}", n, num_arr[n]);
    }

    println!("After add value to array");
    for n in 0..num_arr.len() {
        num_arr[n] += 1;
        println!("New value at num_arr index {} is {}", n, num_arr[n]);
    }

    // Tupple cannot be iterate
    println!("Value at num_tupple {} is {}", 0, num_tupple.0);
    println!("Value at num_tupple {} is {}", 1, num_tupple.1);
    println!("Value at num_tupple {} is {}", 2, num_tupple.2);

    let init_arr = [1; 10]; // 10 elements, with initial 1 value
    println!("init_arr byte is {} bytes", mem::size_of_val(&init_arr));

    let init_arr = [1u16; 10];
    println!("init_arr byte is {} bytes", mem::size_of_val(&init_arr));
}

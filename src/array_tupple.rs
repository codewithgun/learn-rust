pub fn test() {
    let num_arr: [u32; 5] = [1, 2, 3, 4, 5];
    let num_tupple: (u32, char, bool) = (100, 'S', true);

    for n in 0..=4 {
        println!("Number at num_arr index {} is {}", n, num_arr[n]);
    }

    // Tupple cannot be iterate
    println!("Value at num_tupple {} is {}", 0, num_tupple.0);
    println!("Value at num_tupple {} is {}", 1, num_tupple.1);
    println!("Value at num_tupple {} is {}", 2, num_tupple.2);
}

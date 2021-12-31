pub fn test() {
    let x: u128 = 1000;
    // x = u128::checked_add(x, 1000).unwrap();
    // x = u128::checked_sub(x, 5000).unwrap();
    println!("{:?}", x / 3);
    println!("{:?}", x * 900 / 1000);
    println!("{:?}", x * 100 / 1000);
}

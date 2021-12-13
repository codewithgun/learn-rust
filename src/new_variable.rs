pub fn test() {
    // let x = 5;
    let mut x = 5;
    println!("X value before mutate is {}", x);
    // By default, rust variable are immutable, therefore the following line of code will throw error during compilation
    // When "mut" added, the variable will be mutable throughout the program
    x = 6;
    println!("X value after mutate is {}", x);

    // For example, in cases where you’re using large data structures,
    // mutating an instance in place may be faster than copying and returning newly allocated instances.
    // With smaller data structures, creating new instances and writing in a more functional programming style may be easier to think through,
    // so lower performance might be a worthwhile penalty for gaining that clarity.

    // // addition
    // let sum = 5 + 10;

    // // subtraction
    // let difference = 95.5 - 4.3;

    // // multiplication
    // let product = 4 * 30;

    // // division
    // let quotient = 56.7 / 32.2;
    // let floored = 2 / 3; // Results in 0

    // // remainder
    // let remainder = 43 % 5;

    // println!("Sum is {}", sum);
    // println!("Difference is {}", difference);
    // println!("Product is {}", product);
    // println!("Quotient is {}", quotient);
    // println!("Floor is {}", floored);
    // println!("Remainder is {}", remainder);
}

pub fn test() {
    // Statement do not return
    let _x = 5;

    // Expression return
    let x = {
        let y = 3;
        y // last code without ";" is expression
          // Expressions do not include ending semicolons, If you add a semicolon to the end of an expression, you turn it into a statement which will then not return a value
    };
    let num1 = 10;
    let num2 = 20;
    println!("X value is {}", x);
    println!("{} + {} = {}", num1, num2, addition(num1, num2));

    let condition = true;
    let number = if condition { 100 } else { 200 };
    println!("Number should be 100, actual = {}", number);
}

pub fn if_else_test() {
    let temp = 30;
    println!(
        "Today temperature is {}",
        if temp > 25 {
            if temp < 30 {
                "Windy"
            } else if temp < 35 {
                "Warm"
            } else {
                "Hot"
            }
        } else {
            "Cold"
        }
    );
}

// Expression
pub fn addition(num1: u32, num2: u32) -> u32 {
    num1 + num2
}

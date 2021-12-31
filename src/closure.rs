fn plus<F>(x: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

pub fn test() {
    let num1 = 5;
    let one = 1;
    // Closure holding 'one' variable value, closure_fn still have the value of 'one' even though it was at different context
    let closure_fn = |x: i32| -> i32 { x + one };
    print!("{} + {} = {}", num1, one, plus(num1, closure_fn));
}

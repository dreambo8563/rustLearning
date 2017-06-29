fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //   const MAX_POINTS: u32 = 100_000;
    // let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!(
        "{},{},{},{},{}",
        sum,
        difference,
        product,
        quotient,
        remainder
    );
}

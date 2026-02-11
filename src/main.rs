// use std::io;

// create a function to add to numbers?
// fn main() {
//     println!("Hello, world!");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read line");
//     println!("You entered: {}", input);
//     let a: u32 = input.trim().parse().expect("Please enter a number");
//     input.clear();
//     io::stdin().read_line(&mut input).expect("Failed to read line");
//     let b: u32 = input.trim().parse().expect("Please enter a number");
//     let sum = add(a, b);
//     input.clear();
//     println!("The sum of {} and {} is {}", a, b, sum);

// }

// fn add(a:u32, b:u32) -> u32 {
//     a + b
// }


//  2. Factorial of a Number
//  Write a function that calculates the factorial of n number.
//  Example: `factorial(5) = 120`.

// fn main() {
//     println!("Hello, world!");
//     let n = 6;
//     let result = factorial(n);
//     println!("The factorial of {} is {}", n, result);
// }

// fn factorial(n: u32) -> u32 {
//     if n == 0 {
//         1
//     }else {
//         let mut fact = 1;
//         for i in 1..=n {
//             fact = fact * i;
//         }
//         fact
//     }
// }

/*
Write a program that finds the summation of every number from 1 to num. The number will always be a positive integer greater than 0.

For example (Input -> Output):

2 -> 3 (1 + 2)
8 -> 36 (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8)
*/

pub fn main() {
    println!("{}", summation(2));
    println!("{}", summation(8));
}

fn summation(n: i32) -> i32 {
    let mut r = 0;
    for i in 1..=n {
        r = r + i
    }
    r
}

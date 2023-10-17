/*
Create a function with two arguments that will return an array of the first n multiples of x.

Assume both the given number and the number of times to count will be positive numbers greater than 0.

Return the results as an array or list ( depending on language ).

Examples
count_by(1, 10) // should return vec![1,2,3,4,5,6,7,8,9,10]
count_by(2,5) // should return vec![2,4,6,8,10]
*/
pub fn main() {
    println!("{:?}", count_by(1, 10));
    println!("{:?}", count_by(2, 5));
}

fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    for m in 0..n {
        v.push((m + 1) * x)
    }
    v
}

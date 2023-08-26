/*
Define a function that takes an integer argument and returns a logical value true or false depending on if the integer is a prime.

Per Wikipedia, a prime number ( or a prime ) is a natural number greater than 1 that has no positive divisors other than 1 and itself.

Requirements
You can assume you will be given an integer input.
You can not assume that the integer will be only positive. You may be given negative numbers as well ( or 0 ).
NOTE on performance: There are no fancy optimizations required, but still the most trivial solutions might time out. Numbers go up to 2^31 ( or similar, depending on language ). Looping all the way up to n, or n/2, will be too slow.
Example
is_prime(1)  false
is_prime(2)  true
is_prime(-1)  false
*/
pub fn main() {
    println!("{}", is_prime(0));
    println!("{}", is_prime(1));
    println!("{}", is_prime(2));
    println!("{}", is_prime(3));
    println!("{}", is_prime(5));
    println!("{}", is_prime(7));
    println!("{}", is_prime(10));
    println!("{}", is_prime(11));
}

fn is_prime(x: i64) -> bool {
    if x < 2 {
        return false;
    }
    let sqrt = (x as f64).sqrt() as i64;
    for i in 2..=sqrt {
        if x % i == 0 {
            return false;
        }
    }
    true
}
/*
Complete the solution so that it splits the string into pairs of two characters.
If the string contains an odd number of characters 
then it should replace the missing second character of the final pair with an underscore ('_').

Examples:

* 'abc' =>  ['ab', 'c_']
* 'abcdef' => ['ab', 'cd', 'ef']
*/
fn main() {
    println!("Hello, world!");
    println!("{:?}", solution(""));
    println!("{:?}", solution("abc"));
    println!("{:?}", solution("abcdef"));
}

fn solution(s: &str) -> Vec<String> {
    let mut chars: Vec<char> = s.chars().collect();
    let mut v = Vec::new();
    while chars.len() > 0 {
        let mut s = String::new();
        s.push(chars.remove(0));
        if chars.len() > 0 {
            s.push(chars.remove(0));
        } else {
            s.push('_');
        }
        v.push(s);
    }
    v
}
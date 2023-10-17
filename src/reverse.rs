/**
Complete the solution so that it reverses the string passed into it.
'world'  =>  'dlrow'
'word'   =>  'drow'
 */
pub fn main() {
    println!("{}", solution("world"));
    println!("{}", solution("word"))
}

fn solution(phrase: &str) -> String {
    let mut rev = String::new();
    for c in phrase.chars() {
        rev = c.to_string() + &rev;
    }
    return rev;
}

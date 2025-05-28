fn main() {
    // NOTE: Find first word 
    let s = String::from("Hello World");
    let first_word = fine_first_word(s);
    println!("The first is:: {}", first_word);
}

fn fine_first_word(s: String) -> String {
    let mut ans = String::from("");

    for i in s.chars() {
        if i == ' ' {
            break;
        }
        ans += &i.to_string();
    }
    ans
}

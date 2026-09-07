fn reversed_words(text: &str) -> String {
    text.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

fn main() {
    println!("{}", reversed_words("one two three"));
}

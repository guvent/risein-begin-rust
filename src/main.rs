fn main() {
    let s1 = &mut String::from("Hello,");
    let s2 = String::from(" World");

    let concat = concatenate_strings(s1, s2);

    println!("Concat: {}", concat);
}

fn concatenate_strings(s1: &mut String, s2: String) -> &String {
    s1.push_str(s2.as_str());
    s1
}
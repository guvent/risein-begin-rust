fn main() {
    ownership_concept();

    borrowing_references();

    clone_function();
}

fn ownership_concept() {
    let s1 = String::from("Hello"); // stored on the heap
    let s2 = &s1; // clone object

    // let s3 = s1; // ownership of y is moved to z

    println!("Strings: {} -- {}", s1, s2);

    let x = 2; // stored on the stack
    let y = "ok"; // stored on the heap
    let z = y; // stored on the stack

    println!("{} - {} - {}", x, y, z);
}

fn borrowing_references() {
    let my_str = String::from("Hello, World");
    let my_ref = &my_str;
    println!("My Reference: {}", my_ref);

    let my_string = String::from("Hello, Bunny");
    print_str(&my_string);
    println!("String: {}", my_string);

    let mut my_str1 = String::from("hello");
    change_str(&mut my_str1);
    println!("Changed: {}", my_str1);

    fn print_str(s: &String) {
        println!("Print String: {}", s);
    }

    fn change_str(s: &mut String) {
        s.push_str(" world!");
    }

    let mut my_str2 = String::from("Hello, Ref");

    let mut_ref1 = &mut my_str2;
    println!("Mutable First: {}", mut_ref1);

    //// cannot multiple mutable reference from same mutable object!
    // let mut_ref2 = &mut my_str2;
    // println!("Mutable First: {}", mut_ref2);

    let immut_ref1 = &my_str1;
    let immut_ref2 = &my_str1;
    println!("Immutable First: {} --- Second: {}", immut_ref1, immut_ref2);

}


fn clone_function() {
    let original_str = String::from("Hello, world");
    let cloned_str = original_str.clone();

    println!("Original String: {}", original_str);
    println!("Cloned String: {}", cloned_str);

    let original = String::from("Original String");
    let modified = modify_str(&original);

    println!("Original: {}", original);
    println!("Modified: {}", modified);

    fn modify_str(s: &String) -> String {
        let mut clone_str = s.clone();
        clone_str.push_str(" - Modified!");
        clone_str // <- return value and not end of ";" character...
    }
}

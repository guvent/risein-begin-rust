use rand::*;
use std::*;
use rand::distributions::Alphanumeric;

fn main() {
    let s : String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    println!("Hello, world! {}", s);

    variables();

    data_types();

    functions_flow();
}

fn variables() {
    let message = "Demo";

    let a: char = 'o';

    let x: u32 = 33;
    let y: f64 = 3.14;
    let z: bool = false;

    fn add(x: i32, y: i32) -> i32 {
        return x + y;
    }

    let x = 44;

    if x >= 0 {
        println!("Is Positive!");
    } else {
        println!("Is Negative!");
    }

    let mut i = 1;

    while i <= 5 {
        println!("-> {}", i);
        i += 1;
    }

}

fn data_types() {

    //// boolean...
    let _first_bool = true;
    let _second_bool: bool = false;


    //// integers...
    let day_of_weeks: i8 = 7;

    let _number_of_users: i64 = 120000;

    let _number_of_tokens: u64 = 1000;

    // default i32 integer...
    let _just_a_number = 0;


    //// float..
    let _pi: f32 = 3.14;


    //// chars...
    let _character: char = 'a';


    //// strings...
    let _name: &str = "Hello, world!";

    // default &str string...
    let _nick = "quiz...";

    // String object...
    let _user = String::from("Demo, String...");
    // &str string...
    let _string = String::from("Demo, String...").as_str();
    // mut &str string...
    let _mut_string = String::from("Demo, String...").as_mut_str();


    //// &str array...
    let _nums: [i32; 10] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0
    ];

    let _days: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday"
    ];

    // select array item...
    let _first_day = _days[0];
    let _last_day = _days[_days.len()-1];

    // array slice...
    // using "&" character all arrays and objects...
    let _sliced = &_days[1..3];
    let _first_sliced = _sliced[0];


    //// tuples
    let _persons = ("Demo", "Test", "John", "Alice");
    println!("Person: {}", _persons.0);
    println!("Person: {}", _persons.1);
    println!("Person: {}", _persons.2);
    println!("Person: {}", _persons.3);

    let _id_infos = ("Lily", 30);
    let _id_name = _id_infos.0;
    let _id_age = _id_infos.1;
    println!("Name ({}), Age {}", _id_name, _id_age);

    // multiple tuples...
    let _profile_list = (("Alice", 20), ("John", 33), false);

    let _profile_first_name = _profile_list.0.0;
    let _profile_first_age = _profile_list.0.1;
    println!("Name ({}), Age {}", _profile_first_name, _profile_first_age);

    let _profile_second_name = _profile_list.1.0;
    let _profile_second_age = _profile_list.1.1;
    println!("Name ({}), Age {}", _profile_second_name, _profile_second_age);

    let _profile_list_eof = _profile_list.2;
    println!("End of list: {}", _profile_list_eof);


    //// mutable i32
    let mut num = 1;
    num = 2;

    println!("Mutable: {}", num);

    /*
        Booleans: bool, which can have the values true or false
        Integers: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, which are signed and unsigned integers of different sizes
        Floating-point numbers: f32 and f64, which are single-precision and double-precision floating-point numbers, respectively
        Characters: char, which represents a single Unicode character
        Strings: &str and String, which represent a sequence of Unicode characters
        Arrays: [T; N], which represent a fixed-size array of elements of type T
        Slices: [T] and &[T], which represent a variable-size sequence of elements of type T
        Tuples: (T1, T2, ..., Tn), which represent a fixed-size sequence of elements of different types
        Unit type: (), which represents an empty tuple and is used when no value is needed
    */
}

fn functions_flow() {
    let _day_of_week = "Sunday";

    if _day_of_week == "Monday" {
        println!("The race day!");
    } else if _day_of_week == "Friday" {
        println!("Qualifying day!");
    } else {
        println!("Bad Day!!");
    }

    let sum = add(3,4);
    println!("Result: {}", sum);

    fn add(x: i32, y: i32) -> i32 {
        let result = x + y;
        return result;
    }

    fn no_param() -> i32 {
        println!("It's Works");
        1 // <- return value...
    }

    //// For, Loop, While, Match ...
    let mut counter = 0;

    // while
    while counter <= 5 { // 0,1,2,3,4,5
        println!("Counter: {}", counter);
        counter += 1;
    }

    counter = 0;

    // loop
    loop { // 0,1,2,3,4
        println!("Counter: {}", counter);
        counter += 1;

        if counter >= 5 {
            break;
        }
    }

    // for in range
    for number in 1..5 { // 1,2,3,4
        println!("Number 1: {}", number);
    }

    // for in range eq
    for number in 1..=5 { // 1,2,3,4,5
        println!("Number 2: {}", number);
    }

    // for in array
    for number in [1,2,3,4,5] { // 1,2,3,4,5
        println!("Number 3: {}", number);
    }

    let numbers: [i32; 5] = [1,2,3,4,5];

    // for in predefined array
    for number in numbers { // 1,2,3,4,5
        println!("Number 4: {}", number);
    }

    let select = 0;

    //// match
    match select {
        0 => {
            println!("Zero");
            println!("Zero is correct!");
        }
        1 => println!("One"),
        _ => println!("No Match!")
    }

    let choose = match select {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        _ => "Invalid"
    };

    println!("Choose: {}", choose);

}

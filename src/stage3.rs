///// STRUCTS //////

fn structs() {
    let mut person = Person {
        name: String::from("John"),
        gender: String::from("Male"),
        birth: 1992
    };

    println!(
        "Person - ({}) Male: {} Birth: {}",
        person.name, person.gender, person.birth
    );

    person.birth = 1987;

    println!(
        "Person - ({}) Male: {} Birth: {}",
        person.name, person.gender, person.birth
    );

    fn get_person(person: Person) -> [String; 3] {
        [
            person.name,
            person.gender,
            person.birth.to_string()
        ]
    }

    let person_data = get_person(person);

    // :? need to derive on top struct...
    println!("Person: {:?}", person_data);

    fn create_person(name: String, gender: String, birth: u32) -> Person {
        Person {
            name,
            gender,
            birth
        }
    }

    let my_person = create_person(
        "Alice".to_string(),
        "Female".to_string(),
        2000
    );

    // :? need to derive on top struct...
    println!("My Person: {:?}", my_person);

    let t_person = TuplePerson(
        "Mina".to_string(), "Female".to_string(), 2001
    );

    println!(
        "Tuple Person: ({}) Gender: {} Birth: {}",
        t_person.0, t_person.1, t_person.2
    );

    println!("Tuple Person is {:?}", t_person);

    let tuple_peoples = TuplePeoples;

    let a_rectangle = Rectangle {
        with: 4.4,
        height: 7.4
    };

    let r_area = a_rectangle.area();

    println!("Rectangle area: {}", r_area);
}

// derive Debug: dump struct data with :? marks in print command.
#[derive(Debug)]
struct Person {
    name: String,
    gender: String,
    birth: u32,
}

#[derive(Debug)]
struct TuplePerson(String, String, u32);

struct TuplePeoples;

struct Rectangle {
    with: f32,
    height: f32
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.with * self.height
    }
}



///// ENUMS //////

fn main() {
    let weather = Weather::Cloudy;

    println!("Weather is: {:?}", weather);

    let message = Message::Write("Demo Message".to_string());

    process_message(message);

    fn process_message(msg: Message) {
        match msg {
            Message::Quit => {
                println!("Process exited...");
            }
            Message::Move { x, y } => {
                println!("Moved item to x:{}, y:{}", x, y);
            }
            Message::Write(text) => {
                println!("Message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Set color is R:{} G:{} B:{}", r, g, b);
            }
        }
    }

    let animal = Animal::DOG("Golden".to_string());

    if let Animal::DOG(name) = animal {
        println!("My pet name is: {}", name);
    } else {
        println!("My pet is not a Dog!");
    }

    let msg = Message::Write("Some message here...".to_string());
    msg.call();
}

#[derive(Debug)]
enum Weather {
    Cloudy,
    Sunny,
    Rainy,
    Snowy
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Process exited...");
            }
            Message::Move { x, y } => {
                println!("Moved item to x:{}, y:{}", x, y);
            }
            Message::Write(text) => {
                println!("Message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Set color is R:{} G:{} B:{}", r, g, b);
            }
        }
    }
}

enum Animal {
    DOG(String),
    CAT(String),
    BIRD(String),
    FISH(String)
}



///// OPTIONS & RESULT //////

fn options_and_results() {
    let number = -4.0;
    let sqrt_root = find_sqrt_root(number);

    match sqrt_root {
        None => {
            println!("Result is not available!");
        }
        Some(res) => {
            println!("Result is: {}", res);
        }
    }

    let div = divide(3.3, 5.4);

    match div {
        Ok(res) => {
            println!("Division is: {}", res)
        }
        Err(err) => {
            println!("Division impossible!");
        }
    }

    let base = get_from_db("base");
    let height = get_from_db("height");
    let area = calc_triangle_area(base, height);

    match area {
        Ok(a) => {
            print!("Area is: {}", a);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

}

fn find_sqrt_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero is not allowed!".to_string())
    } else {
        Ok(a / b)
    }
}

fn get_from_db(key: &str) -> Option<f64> {
    let db: [(&str, Option<f64>); 2] = [
        ("base", Some(4.5)),
        ("height", Some(3.3))
    ];

    for (k,v) in db {
        if k == key {
            return v;
        }
    }
    None
}

fn calc_triangle_area(base: Option<f64>, height: Option<f64>) -> Result<f64, String> {
    match (base, height) {
        (Some(b), Some(h)) => {
            if b <= 0.0 || h <= 0.0 {
                Err("Both base and height must positive numbers!".to_string())
            } else {
                Ok(0.5 * b * h)
            }
        }
        (None, _) => Err("The base is missing!".to_string()),
        (_, None) => Err("The height is missing!".to_string()),
    }
}



///// COMMON COLLECTIONS //////

use std::collections::HashMap;

fn hashmap() {
    let mut scores = HashMap::new();

    scores.insert("Alice".to_string(), 10);
    scores.insert("John".to_string(), 21);
    scores.insert("Bob".to_string(), 46);

    let alice_score = scores.get(&"Alice".to_string());

    println!("Alice score: {:?}", alice_score);

    println!("{:?}", scores);

    scores.remove(&"John".to_string());

    println!("{:?}", scores);

    for (k, v) in &scores {
        println!("{} {}", k, v);
    }
}

fn strings() {
    let mut first_str = String::from("demo");
    let mut second_str = "Second String".to_string();

    first_str.push_str(", test");

    println!("{}", first_str);

    for c in first_str.chars() {
        println!("{}", c);
    }

    for b in second_str.bytes() {
        println!("{}", b);
    }
}

fn vectors() {
    let mut _numbers = vec![1,2,3,4,5];

    let mut names: Vec<String> = Vec::new();

    names.push("Alice".to_string());
    names.push("Bob".to_string());
    names.push("John".to_string());

    let first_name = &names[0];
    let second_name = &names[1];

    println!("Name - First: {}, Second: {}", first_name, second_name);

    names.pop();

    for name in names {
        println!("Name: {}", name);
    }

    let slice = &_numbers[1..3]; // [2,3]

    // don't need derive(Debug) for array object...
    println!("{:?}", slice)
}



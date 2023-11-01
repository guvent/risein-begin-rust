static something: &str = "some thing...";

/*

Best Practices for Using Lifetimes Effectively in Rust

Now, to the advice section of our saga. Lifetimes are not just a fancy Rust feature to show off at parties. They are a key part of writing safe and efficient code. Here are a few tidbits to munch on:

Don't Fight the Borrow Checker: The borrow checker is your friend. Yes, it can seem overly strict and a bit of a party pooper, but it's just trying to keep your code safe. Work with it, not against it.
Embrace Lifetime Elision: Rust is pretty smart at inferring lifetimes. So, let it do its thing. Explicit lifetime annotations are important but let's not overdo them. Like a well-seasoned dish, balance is key.
Keep it Simple: Lifetimes can get complicated. Try to keep your code as simple and straightforward as possible. If you find yourself tangled in a web of lifetimes, it might be time to step back and reconsider your approach.

*/

struct Ancestor<'a>(&'a str);

///// Lifetime Issues and Solutions
///// Common Issues with Lifetimes

// fn problematic_function() -> &'static str {
//     let string_inside = String::from("Hello, Rust!");
//     // &string_inside[..]
//     // ^-------------^^^^
//     // ||
//     // |`string_inside` is borrowed here
//     // returns a value referencing data owned by the current function
// }

///// Strategies for Resolving Lifetime Issues
fn better_function() -> String {
    let string_inside = String::from("Hello, Rust!");
    string_inside
}

// Outlives Relationship in Lifetimes...
fn ancestor() {
    let name = String::from("Great-Grandpa");
    // let person: Ancestor<'static> = Ancestor(&name); // Error! `name` doesn't live long enough
    // Ancestor: type annotation requires that `name` is borrowed for `'static`
}

////// Explanation of Lifetime Bounds //////
struct LongLived<'a>(&'a str);

struct ShortLived<'a> {
    name: &'a str,
    long: LongLived<'a>
}
////// Explanation of Lifetime Bounds //////

fn borrowed() {
    let parent: &'static str =
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";

    // let kid: &'static str = {
    //     let short_text = String::from("lorem ipsum dolor");
    //     &short_text <- borrowed value does not live long enough...
    // };
}

// Static Lifetimes
fn static1() {
    let name = "Ben 10";
    let person = Person { name };

    let s: &'static str = "ok";
}


// Lifetime Annotations in Structs
struct Person<'a> {
    name: &'a str,
}

// Examples of Functions with Explicit Lifetime Annotations
fn no_no_fn<'a>(x: &'a str, y: &'a str) -> &'a str {
    let some_str = String::from(x);
    // &some_str <- incorrect from lifetime...
    &""
}

///// Examples of Lifetime Elision /////
struct Sentence<'a> {
    content: &'a str,
}

impl<'a> Sentence<'a> {
    fn yep(&self) -> &str {
        "do not code until 3.00AM..."
    }
}
///// Examples of Lifetime Elision /////

// Annotate Lifetimes in Function Parameters and Return Types
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// Syntax of Lifetime Annotations
fn mixes<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    let some = String::from(x);
    // &some <- incorrect from lifetime...
    &""
}

// Syntax of Lifetime Annotations
fn mix<'a>(x: &'a str) -> &'a str {
    let some = String::from(x);
    // &some <- incorrect from lifetime...
    &""
}


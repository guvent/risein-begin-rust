

/*

FnOnce: This trait represents closures that can be called exactly once.
        They may move (consume) values from their environment.
        It's like a one-time party invitation – you use it, and it's gone.

FnMut:  This trait is for closures that can be called multiple times
        and can mutate values from their environment.
        It's like having a key to the house – you can enter as many times as you want,
        and you're allowed to move the furniture.

Fn:     This trait is for closures that can be called multiple times without
        mutating their environment.
        It's like being a ghost – you can pass through the house as much as you want,
        but you can't change anything.

*/

fn callback_lambda() {
    fn download_data(url: &str, callback: impl FnOnce(&str)) {
        println!("Connecting from {}...", url);

        std::thread::sleep(std::time::Duration::from_secs(1));

        let data = format!("URL: {}\nPayload: Success...", url);

        callback(&data);
    }

    download_data("https://www.rust-lang.org", |data: &str| {
        println!("Received data:\n{}", data);
    });
}

fn closures() {
    let lambda = || println!("this is lambda!");
    lambda();

    let even_nums = |x: i32| x % 2 == 0;
    let is_even = even_nums(9);
    println!("Is Even: {}", is_even);

    let ret_val = |x: i32| -> i32 { x * 2 };
    let is_val = ret_val(44);
    println!("Value: {}", is_val);

    let numbers = vec![1,2,3,4,5,6,7];
    let vec_nums: Vec<i32> = numbers
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect();
    print!("Evens: {:?}", vec_nums);
}


// same function like that;
// let even_nums = |x: i32| x % 2 == 0;
fn even_nums_same(numbers: Vec<i32>) -> Vec<i32> {
    let mut even_nums = Vec::new();

    for number in numbers {
        if number % 2 == 0 {
            even_nums.push(number);
        }
    }

    even_nums
}


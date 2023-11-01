
///// ADVANCED TRAITS //////

fn vec_box_traits() {
    let mut cars: Vec<Box<dyn Cars>> = Vec::new();

    cars.push(Box::new(SUV{
        speed: 100,
        top: 200
    }));

    cars.push(Box::new(SUV{
        speed: 140,
        top: 210
    }));

    for car in cars {
        println!("Car speed: {}", car.go());
    }
}

fn box_dyn_traits() {
    let my_car: Box<dyn Cars> = Box::new(SUV{
        speed: 100,
        top: 200
    });

    println!("{:?}", my_car.go());

    stop_my_car(my_car);

}

trait Cars {
    fn go(&self) -> u32;
    fn stop(&self);
}

struct SUV {
    speed: u32,
    top: u32
}

impl Cars for SUV {
    fn go(&self) -> u32 {
        self.speed + 10
    }

    fn stop(&self) {
        println!("Cars stopped! {}", self.speed - self.speed);
    }
}

fn pointer() {
    let mut ptr = Box::new(128);

    //ptr = 10; <- not allowed...
    *ptr = 10;

    println!("{}", ptr);
}

fn scope() {
    let b = Box::new("Alice");

    println!("{}", b);

    {
        let t = Box::new("Temp");
        println!("{}", t);
    }

    // t is out of scope...
    // println!("{}", t);


}

fn stop_my_car(car: Box<dyn Cars>) {
    println!("Cars stopping now...");
    car.stop()
}


///// INTRODUCTION GENERIC USAGE //////

fn swap<T: Copy>(x: &mut T, y: &mut T) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

trait Summary {
    fn summarize(&self) -> String;
}

fn print_summary<T: Summary>(item: T) {
    print!("{}", item.summarize());
}

fn print_double_summary<T, U>(a: T, b: U)
    where
        T: Summary,
        U: Summary + Clone
{
    println!("{}", a.summarize());
    println!("{}", b.summarize());
    let cloned = b.clone();
    print!("{}", cloned.summarize());
}

// enum Option<T> {
//     Some(T),
//     None
// }
//
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }





///// IMPLEMENTATION GENERICS USING STRUCT AND ENUM //////

fn implement_type_struct() {
    let mut honda = Electrical { battery_level: 64 };
    let mut ford = Gas { gas_level: 24 };

    honda.refuel(36.0);
    ford.refuel(22.0);

    println!("Honda Charge: {}", honda.battery_level);
    println!("Ford Gas: {}", ford.gas_level);
}

trait Vehicle {
    type Fuel;

    fn refuel(&mut self, fuel: Self::Fuel);
}

struct Electrical {
    battery_level: u32,
}

struct Gas {
    gas_level: u32,
}

impl Vehicle for Electrical {
    type Fuel = f32;

    fn refuel(&mut self, fuel: Self::Fuel) {
        self.battery_level += (fuel * 100.0) as u32;
    }
}

impl Vehicle for Gas {
    type Fuel = f32;

    fn refuel(&mut self, fuel: Self::Fuel) {
        self.gas_level += (fuel * 100.0) as u32;
    }
}

fn container() {
    let int_cont = Container { value: 889 };
    let str_cont = Container { value: "ok" };
    let bool_cont = Container { value: true };

    println!("{}", int_cont.value);
    println!("{}", str_cont.value);
    println!("{}", bool_cont.value);

    let success_res : MagicalRes<i32, String> = MagicalRes::Success(333);
    let failed_res : MagicalRes<i32, String> = MagicalRes::Failed("Error found!".to_string());

    match success_res {
        MagicalRes::Success(v) => println!("Success: {}", v),
        MagicalRes::Failed(e) => println!("Failed: {}", e),
    }

    match failed_res {
        MagicalRes::Success(v) => println!("Success: {}", v),
        MagicalRes::Failed(e) => println!("Failed: {}", e),
    }

}

enum  MagicalRes<T, E> {
    Success(T),
    Failed(E)
}

struct Container<T> {
    value: T
}



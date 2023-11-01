
///// TRAITS //////

fn traits() {
    let audi = &Audi {
        name: "Audi".to_string(),
        color: "Red".to_string(),
        price: 120000.0
    };

    let mercedes = &Mercedes {
        name: "Mercedes".to_string(),
        color: "Gray".to_string(),
        price: 140000.0
    };

    let land_rover = &LandRover {
        name: "Land Rover".to_string(),
        color: "Black".to_string(),
        price: 230000.0
    };

    let car1 = Cars::buy(audi);
    let car2 = Cars::buy(mercedes);
    let car3 = Cars::buy(land_rover);

    println!("{:?}", &car1);
    println!("{:?}", &car2);
    println!("{:?}", &car3);

    let a_car = buy_a_car(audi);

    match a_car {
        None => println!("None selected a car!"),
        Some(item) => println!("{}", item)
    }

    SUV::get_bag(land_rover);
    Automotive::get_speed(mercedes);

    let buy_suv = <dyn SUV>::buy(land_rover);
    let buy_automotive = <dyn Automotive>::buy(mercedes);

    println!("Buy SUV {:?}", buy_suv.unwrap());
    println!("Buy Automotive {:?}", buy_automotive.unwrap());
}

trait Cars {
    fn buy(&self) -> Option<String>;
}

struct Audi {
    name: String,
    color: String,
    price: f64
}

struct Mercedes {
    name: String,
    color: String,
    price: f64
}

struct LandRover {
    name: String,
    color: String,
    price: f64
}

impl Cars for Audi {
    fn buy(&self) -> Option<String> {
        Some(format!(
            "Great!, Buy a {} it color {} and price {}",
            self.name, self.color, self.price
        ))
    }
}

impl Cars for Mercedes {
    fn buy(&self) -> Option<String> {
        Some(format!(
            "Congratulations!, Buy a {} it color {} and price {}",
            self.name, self.color, self.price
        ))
    }
}

impl Cars for LandRover {
    fn buy(&self) -> Option<String> {
        Some(format!(
            "Perfect!, Buy a {} it color {} and price {}",
            self.name, self.color, self.price
        ))
    }
}

fn buy_a_car<T: Cars>(car: &T) -> Option<String> {
    car.buy()
}

trait SUV: Cars {
    fn get_bag(&self);
}

trait Automotive: Cars {
    fn get_speed(&self);
}

impl SUV for LandRover {
    fn get_bag(&self) {
        println!("This car has a bag (500 lt.)");
    }
}

impl Automotive for Mercedes {
    fn get_speed(&self) {
        println!("This car has top speed (280 km/h)");
    }
}

fn trait_string() {
    let my_str = String::from("it is a trait!").display();

    println!("{}", my_str);
}

trait Display {
    fn display(&self) -> String;
}

impl Display for String {
    fn display(&self) -> String {
        format!("Display: {}", self)
    }
}



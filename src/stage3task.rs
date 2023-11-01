
use log::error;

fn task() {

    let add = Operation::Add(4.0,5.0).calc();
    let sub = Operation::Subtract(5.0, 2.2).calc();
    let mul = Operation::Multiply(3.0, 4.0).calc();
    let div = Operation::Divide(4.0, 2.0).calc();

    match add {
        Ok(res) => println!("{}", res),
        Err(err) => println!("Error: {}", err)
    }

    match sub {
        Ok(res) => println!("{}", res),
        Err(err) => println!("Error: {}", err)
    }

    match mul {
        Ok(res) => println!("{}", res),
        Err(err) => println!("Error: {}", err)
    }

    match div {
        Ok(res) => println!("{}", res),
        Err(err) => println!("Error: {}", err)
    }

    let div_err = Operation::Divide(4.0, 0.0).calc();

    match div_err {
        Ok(res) => println!("{}", res),
        Err(err) => println!("Error: {}", err)
    }

}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64)
}

impl Operation {
    fn calc(&self) -> Result<f64, String> {
        match self {
            Operation::Add(a, b) => Ok(a + b),
            Operation::Subtract(a, b) => Ok(a - b),
            Operation::Multiply(a, b) => Ok(a * b),
            Operation::Divide(a, b) => {
                if b <= &0.0 {
                    Err("Second number must than zero!".to_string())
                } else {
                    Ok(a / b)
                }
            }
        }
    }
}

mod sports;

use sports::{football, FootballPlayer};
// use sports::football;
// use sports::FootballPlayer;

use sports::apple;

// Use With Relative path
// use self::sports::apple;

// Use With Absolute path
// use crate::sports::apple;


fn modula() {
    football();

    let icardi = FootballPlayer {
        name: "Icardi".to_string(),
        age: 33,
    };

    println!("{:?}", icardi);

    module::nested_module::show();

    apple::munch();

}

// mod sports {
//     pub fn football() {
//         println!("Let's play football...");
//     }
//
//     pub struct FootballPlayer {
//         pub name: String,
//         pub age: i32
//     }
// }

mod module {
    pub mod nested_module {
        pub fn show() {
            println!("Show nested modula...");
        }
    }
}

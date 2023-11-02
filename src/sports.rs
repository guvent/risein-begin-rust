
// mod sports { <- don't need when separate file...
    pub fn football() {
        println!("Let's play football...");
    }

    #[derive(Debug)]
    pub struct FootballPlayer {
        pub name: String,
        pub age: i32
    }
// }  <- don't need when separate file...

pub mod apple {
    pub fn munch() {
        println!("Munching an apple...");
    }
}

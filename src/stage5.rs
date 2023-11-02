

// Custom Iterator

fn custom_iter() {
    let countdown = Countdown { remaining: 5 };

    for i in countdown {
        println!("Remaining: {}", i);
    }
}

struct Countdown {
    remaining: i32,
}

impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let current = self.remaining;
            self.remaining -= 1;
            Some(current)
        } else {
            None
        }
    }
}

// Iterator Type in Rust

fn iter_vec() {
    let vec = vec![1, 2, 3, 4, 5];
    for item in vec.iter() {
        println!("{}", item);
    }

    // iter_mut: This iterator type is used when you want to iterate over a mutable collection by
    //   borrowing its elements mutably. With iter_mut, you can access and modify the elements in
    //   a collection without consuming the collection itself.
    //   This is useful when you need to make changes to the elements in a collection.
    let mut vec2 = vec![1, 2, 3, 4, 5];
    for item in vec2.iter_mut() {
        *item *= 2;
    }

    // into_iter: This iterator type is used when you want to iterate over a collection by
    //   consuming it. With into_iter, you can access the elements in a collection and
    //   consume them, which means that the collection will no longer be available
    //   after the iteration.
    let vec3 = vec![1, 2, 3, 4, 5];
    for item in vec3.into_iter() {
        println!("{}", item);
    }
}

// The Iterator trait

// trait Iterator {
//     type Item;
//
//     fn next(&mut self) -> Option<Self::Item>;
// }

fn iter_trait() {
    let mut fib = Fibonacci { current: 0, next: 1 };

    for _ in 0..10 {
        println!("{}", fib.next().unwrap());
    }
}

struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

fn iterators() {
    let numbers = vec![1, 2, 3, 4, 5];

    let mut iter = numbers.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), None);
}
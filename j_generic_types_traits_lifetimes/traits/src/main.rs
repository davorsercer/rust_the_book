use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let pair = Pair::new(1, 2);
    pair.cmp_display();

    let pair = Pair::new("hello", "world");
    pair.cmp_display();
}
// The code defines a generic struct `Pair` that holds two values of the same type.
// It implements a method `new` to create a new instance of `Pair`.
// It also implements a method `cmp_display` that compares the two values and prints the larger one.

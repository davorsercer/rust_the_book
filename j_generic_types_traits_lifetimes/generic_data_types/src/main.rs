fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}

/*
The error you’re seeing is because you’re trying to use the > operator on a generic type T, but Rust doesn’t know if T can actually be compared like that. Not all types support comparison by default.

To fix this, you need to add a trait bound that says T must implement the PartialOrd trait, which allows comparison with operators like <, >, etc
*/

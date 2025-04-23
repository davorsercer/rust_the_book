// An Example Program Using Structs
// Method Syntax
// Using methods from struct insted of function
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle  {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    )
}

// // Printing struct
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {rect1:?}");
// }


// // Cal. area with struct
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


// // Cal. area with tuple
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// // Cal. area with variables
// fn main() {
//     let width1: u32 = 30;
//     let height1: u32 = 50;

//     println!("The area of the rectangle is {} square pixels.", 
//      area(width1, height1));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// // Tuple structs
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     println!("Black: ({}, {}, {})", black.0, black.1, black.2);
//     println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);
// }


// // Structs
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     // --snip--

//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1 // This will move user1 into user2
//     };

//     println!("User 2: {} – {}", user2.username, user2.email);
// }


// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,       // default value
//         username,           // shorthand for (username: username)
//         email,              // shorthand for (email: email)
//         sign_in_count: 1,   // default value
//     }
// }

// fn main() {
//     let user1 = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );
//     println!("User 1: {} - {}", user1.username, user1.email);
// }
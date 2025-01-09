// struct User{
//     username: String,
//     email:String,
//     sign_in_count: u64,
//     active: bool
// }

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}
fn main() {
//    let mut user1 = User{
//     email: String::from("abc@gmail.com"),
//     username: String::from("deepak"),
//     active: true,
//     sign_in_count: 1
//    };

//    let name  = user1.username;
//    user1.username = String::from("chiku");

//    let user2 = build_user(
//     String::from("hyle@123"),
//     String::from("adi")
//    );

//    let user3 = User{
//     email: String::from("kyle@12"),
//     username: String::from("itachi"),
//     ..user2   //ie active: true, sign_in_count: 1
//    };

//exmaple:

let rect:Rectangle = Rectangle{
    width: 30,
    height: 50,
};

println!("rect: {:?}", rect);
println!("The area of rectangle is: {} ", area(&rect));
}

// fn build_user(email: String, username: String) -> User{

//     User{
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }


fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}
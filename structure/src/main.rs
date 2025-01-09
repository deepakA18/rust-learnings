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

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other:  &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size
        }
    }
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

let rect1: Rectangle = Rectangle{
    width: 20, 
    height: 40,
};

let rect2: Rectangle = Rectangle{
    width: 40, 
    height: 50,
};

let rect3:Rectangle = Rectangle::square(25);

println!("The area of rect1 is: {}", rect.can_hold(&rect1));
println!("The area of rect2 is: {}", rect.can_hold(&rect2));
println!("The area of rect3 is: {:?}", rect3);
println!("rect: {:?}", rect);
println!("The area of rectangle is: {} ", rect.area());
}

// fn build_user(email: String, username: String) -> User{

//     User{
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }


// fn area(rectangle: &Rectangle) -> u32{
//     rectangle.width * rectangle.height
// }
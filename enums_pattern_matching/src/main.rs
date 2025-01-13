enum IpAddrKind {
    V4(u32,u32,u32,u32),
    V6(String)
}

enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

struct Quit{}

struct Move{
    x:i32,
    y:i32
}

struct Write(String);
struct MoveMessage{
    x: i32,
    y:i32
}

struct IpAddr{
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California
}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin{
        Coin::Penny =>1,
        Coin:: Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }

}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let localhost = IpAddr{
    //     kind: V4,
    //     address: String::from("127.0.0.1"),
    // }
    let localhost = IpAddrKind::V4(127,0,0,1);

    //*******option enum************

    // enum Option<T>{
    //     Some(T),
    //     None
    // }

    let some_number = Some(5);
    let some_string = Some("string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);  //if no value is set, then will take default value in unwrap method

    let sum = x + y.unwrap_or(0); //cannot add option<i8> to i8


    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //***********if let syntax*******************

    let some_value = Some(3);
    match some_value{
        Some(3) => println!("Three!"),
        _ => ()
    }

    if let Some(3) = some_value {
        println!("three!");
    }

}


fn plus_one(x: Option<i32>) -> Option<i32>{

    match x {
        None => None,
        Some(i) => Some(i + 1),
        _ => None
    }

}
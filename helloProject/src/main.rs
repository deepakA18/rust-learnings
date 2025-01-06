//Functions:
//Entry Point is main
//an function/variables should be written in snake case.

//snake case: hello_world
//kebab case: hello-world



// fn main() {
// hello_world();
// tell_height(167);
// human_id("Deepak", 22, 167.4);

// let _X = {
//     let price= 10;
//     let qty= 5;
//     price * qty
// };
// println!("The result is: {}", _X);

// let y = add(6, 4);
// println!("The sum is: {}", y);

// let weight = 70.0;
// let height = 180.9;

// println!("The bmi is: {:.4}", calculate_bmi(weight, height));

// }

// //rust supports hoisting:
// fn hello_world(){
//     println!("Hello,world")
// }

// fn tell_height(height: u32){
//     println!("My height is: {}", height);
// }

// fn human_id(name: &str, age: u32, height: f32){
//     println!("My name is {}, I am {} yrs old, with height {}", name, age, height);

// }

// //Expressions and Statements:
// //Expression: Anything that returns a value


// //Expression example:
// //5
// //true and false
// //add(3,4)

// //function returning values:

// fn add(a: i32, b:i32) -> i32{
//     a+b
// }


// //Statement: Anything that does not return a statement
// //Almost all statements in Rust ends with ;
// // let y = let x = 10;
// //1. Variable declaration: let x =5;
// //2. Function definition: fn foo() {}
// //3. Control flow statements: if, else, while

// fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {

//     weight_kg / (height_m * height_m)

// }

//Ownership:

//1. Eachg value in rust has an owner.
// fn main(){
//     let s1: String = String::from("Rust");
//     let len = calculate_length(&s1);

//     println!("The len of {} is {}", s1, len);

// }

// fn calculate_length(s: &String)-> usize{
//     s.len()
// }

//2. There can only be one owner at a time.

// fn main(){
//     let s1: String = String::from("RUST");
//     let s2:String = s1;

//     println!("{}", s2); 
// }

//3. When the owner goes out of scope, the value will be dropped:

// fn main(){
//     let s1: String = String::from("Rust");
//         let len = calculate_length(&s1);
    
//         println!("The len of {} is {}", s1, len);
    
//     }

//     //s1 goes out of scope, the value will be dropped:

//     fn print_lost(){
//         println!("The s1 {}",s1)
//     }
    
//     fn calculate_length(s: &String)-> usize{
//         s.len()
//     }

//Borrowing and References:

//Understanding references:

//References: Allows u to borrow values without taking ownership
//Immutable reference 
//Mutable reference
//Create a reference by adding "&"

// fn main(){
//     let mut _x: i32 = 5;
//     let _y: &mut i32 = &mut _x; //just a reference not the owner.

//     *_y += 1;

//     println!("The value of X: {}", _x);
//     // println!("The value of Y: {}", _y);

// }

//Demo of one mutable reference or many mutable references:

fn main(){

    let mut account = BankAccount{
        owner: "Bob".to_string(),
        balance: 160.70
    };

    //Immutable borrow to check the balance:

    account.check_balance();

    //Mutable borrow to withdraw money:

    account.withdraw(50.0);

    //Immutable borrow to check the balance:
    account.check_balance();

    account.withdraw(40.0);

}

struct BankAccount{
    owner: String,
    balance: f64
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){

        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;

    }

    fn check_balance(&self){

        println!("Account owned by {} has a balance of {}", self.owner, self.balance);

    }
}
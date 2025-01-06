fn main() {
   //Ownership rules:

   //1. Each value in rust has a variable that's called it;s owner
   //2. There can only be 1 owner at a time.
   //3. When the owner goes out of scope the value will be dropped

   let x: i32 = 5;
   let y:i32 = x;

   println!("The value of y is {}", y);

   let s1: String = String::from("Hello!");
   let s2:String = s1; //value of s1 is borrowed, use "clone()" method if want a copy
   

   println!("The value of s2 is {}", s2);

   let s: String = String::from("string!");
   takes_ownership(s);
   //here the variable s is dropped as assigned to the param some_string in takes_ownership function
    // println!("the string is: {}", s);

    let x:i32 = 5;
    makes_copy(x);
    println!("The integer is: {}", x);


    let string1: String = gives_ownership();
    println!("This is string1: {}", string1);

    let string2: String = String::from("Benlo!");

    let string3: String = gives_takesback_ownership(string2);
    println!("this is string3: {}", string3);

}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer:i32){
    println!("{}", some_integer);
}

fn gives_ownership() -> String{
    let some_string: String = String::from("hello!");
    some_string  
}

fn gives_takesback_ownership(a_string:String) -> String{
a_string
}


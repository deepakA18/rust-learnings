fn main() {
    let x = 5;
    println!("The value of x is {} ", x);
    let x= 6;  //shadowing a variable 
    println!("The value of x is {}", x);

    //tuples:

    let tup: (&str, i32) = ("Fock it!",100_000);
    let (name, age) = tup;
    let age = tup.34;

}

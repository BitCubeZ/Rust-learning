//Functions
//Use either snake or kebab case.
//Snake: hello_world
//kebab: hello-world
fn main() //The entrypoint  
{ 
    hello_world();
    tell_height(182);
    human_id("Joel",55,182.2);

    let _x: i32 =
    {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
        //return price * qty;
    };

    println!("Result is: {}", _x);
    add(4,6);
    let y: i32 = add(4,6);
    println!("Y is: {}", y);
    println!("Value from add function is: {}", add(4,6));

    //calling BMI
    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi = calculate_bmi(weight, height);
    println!("Your bmi is: {:.2}", bmi);

    ownership();
    borrowing();

    //References and mutables
    let mut account = BankAccount
    {
        owner:"Alice".to_string(),
        balance: 150.55,
    };

    //imutable borrow to check the balance
    account.check_balance();
    
    //mutable borrow to withdraw money
    account.withdraw(50.0);
    account.check_balance();
}

fn hello_world()
{
    println!("Hello, rust");
}

//You can insert input values
fn tell_height(height: u32)
{
    println!("My height is {}", height);
}

fn human_id(name: &str, age: u32, height: f32)
{
    println!("My name is {}, I am {} years old and I am {} cm tall", name, age, height);
}

//Expressions and statements
//An expression is anything that returns a value
//Statements are anything that doesn't return a value

fn add(a: i32, b: i32) -> i32
{
    a+b
}

//Statements:
//Almost all statements in Rust end with ;
// let y = let x = 10;
//Declarations of variables
//Function definitions
//If, else, while etc...

//BMI = height(kg)/height(m)^2

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64
{
    weight_kg / (height_m*height_m)
}

//Ownership
// Every value has a single owner. When the owner goes out of scope, the value gets dropped

fn ownership()
{
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of {} is {}", s1, len);

    //Transfer of ownership
    let s2 = s1;

    println!("{}", s2);

    scope_drop();
}

fn calculate_length(s: &String) -> usize
{
    s.len()
}

fn scope_drop()
{
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of {} is {}", s1, len);
}

//fn print_lost(s: &string) // will result in not being able to find s1
//{
//    println!("{}", &s1);
//}

fn borrowing()
{
    //let mut _x: i32 = 5;
    
    //let _r: &mut i32 = &mut _x;

    //println!("Value of x = {}", _x);
    //println!("Value of r = {}", _r);

    //*_r += 1;

    //println!("Value of r = {}", _r);
}

struct BankAccount
{
    owner: String,
    balance: f64,
}

impl BankAccount  
{
    fn withdraw(&mut self, amount: f64)
    {
        println!("Withdrawing {} from {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self)
    {
        println!("Account owned by {} has the balance of {}", self.owner, self.balance);

    }
}
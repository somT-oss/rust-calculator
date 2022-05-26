use std::io::stdin;

// Calulator struct 
struct Claculator{
    num1: i32,
    num2: i32
}

// Implementation for the Calculator truct
impl Claculator {
    // This is the new function for the struct that defines a base Calculator struct 
    fn new() -> Self {
        Claculator {
            num1: 2,
            num2: 3
        }
    }

    // These are the functionalities the calculator performs
    fn add(&self) -> i32 {
        self.num1 + self.num2
    }
    fn multiply(&self) -> i32 {
        self.num1 * self.num2
    }
    fn subtract(&self) -> i32 {
        self.num1 - self.num2
    }
    fn divide(&self) -> i32 {
        self.num1 / self.num2
    }
}

// The main function that executes the whole calculator actions
fn main() {
    let user_choice = user_choice();

    if user_choice == "Addition" {
        addition();
    }
    else if user_choice == "Subtraction" {
        subtraction();
    }

    else if user_choice == "Multiplication" {
        multiplication();
    }

    else if user_choice == "Division" {
        divide();
    }

    else {
        println!("Your answer is not in the option");
    }
}


// This function handles the logic behind action selction for the Calcultor
fn user_choice() -> String {
    let mut user_answer = String::new();
    
    println!("This is a simple calculator application written in Rust \n");
    println!("There are operations that can be performed on the numbers \n");
    println!("Your answer should be within the options \n");
    println!("Here are the options: 
            \n 1) Addition
            \n 2) Multiplication
            \n 3) Subtraction
            \n 4) Division \n");

    let all_options = ["Addition", "Multiplication", "Subtraction", "Division"];
    stdin().read_line(&mut user_answer).ok().expect("Couldn't read input");

    let trimmed_user_input = &mut user_answer.trim();

    for option in all_options.iter() {
        if option == trimmed_user_input {
            return trimmed_user_input.to_string();
        }
        else {
            continue;
        }
    }

    String::from("")
}

// This function handles the collection of numbers from the user
// It returns a tuple of a boolean and numbers
// Basically, if the user enters a number it returns a tuple in this format (true, int_one, int_two)
// If the user doesn't want to input a number it return (false, 0, 0) 
fn user_input_numbers() -> (bool, i32, i32) {

    println!("Do you wish to choose the numbers for this operation? \n");
    let mut answer = String::new();
    let error_message = "{'Error': 'Could not process answer'}";
    stdin().read_line(&mut answer).ok().expect(error_message);

    let trimmed_answer = &mut answer.trim();

    if trimmed_answer == &"No".to_string() || trimmed_answer == &"no".to_string() {
       (false, 0, 0)
    }
    else if trimmed_answer == &"Yes".to_string() || trimmed_answer == &"yes".to_string() {
        println!("Enter the first number: \n");
        let mut string_frist_number = String::new();

        stdin().read_line(&mut string_frist_number).ok().expect("{'Error': 'Could not process answer'}");
        let int_first_number: i32 = string_frist_number.trim().parse().unwrap();

        println!("Enter the first number: \n");
        let mut string_second_number = String::new();

        stdin().read_line(&mut string_second_number).ok().expect("{'Error': 'Could not process answer'}");
        let int_secodnd_number: i32 = string_second_number.trim().parse().unwrap();

        (true, int_first_number, int_secodnd_number)
    }
    else {
        (false, 0, 0)
    }   

}


// The functions down here handle each arithmetic operation carried out by the calculator
fn addition() {
    let user_input_numbers = user_input_numbers();
    let mut new_calculator = Claculator::new();

    if user_input_numbers.0 ==  true{
        new_calculator.num1 = user_input_numbers.1;
        new_calculator.num2 = user_input_numbers.2;

        let answer = new_calculator.add();
        println!("{} + {} = {}", new_calculator.num1, new_calculator.num2, answer);
    }

    else {
        println!("You are using the default numbers for this calculation");
        println!("The numbers are {}, {}", new_calculator.num1, new_calculator.num2);
        let answer = new_calculator.add();
        println!("{} + {} = {}", new_calculator.num1, new_calculator.num2, answer);
    }
    
}   

fn subtraction() {
    let user_input_numbers = user_input_numbers();
    let mut new_calculator = Claculator::new();

    if user_input_numbers.0 == true {
        new_calculator.num1 = user_input_numbers.1;
        new_calculator.num2 = user_input_numbers.2;

        let answer = new_calculator.subtract();
        println!("{} - {} = {}", new_calculator.num1, new_calculator.num2, answer);
    }
    
    else {
        println!("You are using the default numbers for this calculation");
        println!("The numbers are {}, {}", new_calculator.num1, new_calculator.num2);
        let answer = new_calculator.subtract();
        println!("{} - {} = {}", new_calculator.num1, new_calculator.num2, answer);
    }
    
}   

fn multiplication() {
    let user_input_numbers = user_input_numbers();
    let mut new_calculator = Claculator::new();

    if user_input_numbers.0 == true {
        new_calculator.num1 = user_input_numbers.1;
        new_calculator.num2 = user_input_numbers.2;

        let answer = new_calculator.multiply();
        println!("{} * {} = {}", new_calculator.num1, new_calculator.num2, answer);
    }
    
    else {
        println!("You are using the default numbers for this calculation");
        println!("The numbers are {}, {}", new_calculator.num1, new_calculator.num2);
        let answer = new_calculator.multiply();
        println!("{} * {} = {}", new_calculator.num1, new_calculator.num2, answer);
    }
    
} 

fn divide() {
    let user_input_numbers = user_input_numbers();
    let mut new_calculator = Claculator::new();

    if user_input_numbers.0 == true {
        new_calculator.num1 = user_input_numbers.1;
        new_calculator.num2 = user_input_numbers.2;

        let answer = new_calculator.divide();
        println!("{} * {} = {}", new_calculator.num1, new_calculator.num2, answer);
    }
    
    else {
        println!("You are using the default numbers for this calculation");
        println!("The numbers are {}, {}", new_calculator.num1, new_calculator.num2);
        let answer = new_calculator.divide();
        println!("{} * {} = {}", new_calculator.num1, new_calculator.num2, answer);
    }
    
} 

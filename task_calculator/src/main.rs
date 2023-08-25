use std::io;

fn main() {

    //Define a new string variable to read string input from user
    let mut input1 = String::new();

    // Prompt the user for input
    println!("Please enter the first number:");

    // Read the user's input and store it in the 'input' String
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    //Same Proccess Again
    let mut input2 = String::new();
    println!("Please enter the second number:");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    let mut operation = String::new();
    println!("Please enter an operation to perform: (Add, Subtract, Multiply, Divide)");
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    //Trim the whitspace charachters from inputs and parse it to a float number.
    let number1: f64 = input1.trim().parse().expect("Invalid Input");
    let number2: f64 = input2.trim().parse().expect("Invalid Input");

    //Trim the whitspace charachters from operation.
    let trimmed_operation = operation.trim();

    //Define an Operation instance variable
    let enum_instance: Operation;

    //Check the user input and initialize the enum_instance accordingly
    if trimmed_operation == "Add".to_string() {
        enum_instance = Operation::Add {
            x: number1,
            y: number2,
        };
    } else if trimmed_operation == "Subtract".to_string() {
        enum_instance = Operation::Subtract {
            x: number1,
            y: number2,
        };
    } else if trimmed_operation == "Multiply".to_string() {
        enum_instance = Operation::Multiply {
            x: number1,
            y: number2,
        };
    } else if trimmed_operation == "Divide".to_string() {
        enum_instance = Operation::Divide {
            x: number1,
            y: number2,
        };
    } else {
        println!("Invalid Operation. Please check your inputs");
        return;
    }

    //Calculate the result
    let result = calculate(enum_instance);

    //Print the result
    println!("Result of the calculation: {}" ,result);

    

}

//Operation Enum
enum Operation {
    Add { x: f64, y: f64 },
    Subtract { x: f64, y: f64 },
    Multiply { x: f64, y: f64 },
    Divide { x: f64, y: f64 },
}

//Function for calculation
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add { x, y } => return x + y,
        Operation::Subtract { x, y } => return x - y,
        Operation::Multiply { x, y } => return x * y,
        Operation::Divide { x, y } => return x / y,
    }
}

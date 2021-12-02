use std::io::{stdin, stdout, Write};



#checks proper input
fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}


fn main() {
    
    #starts loop
    loop {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator = String::new();
        
        #these next two parts take input.
        print!("what is the first number?: ");
        read(&mut num1);

        print!("what is the second number?: ");
        read(&mut num2);

        #Requests which operation the operator wants to do
        print!("what operation would you like to do? [+-*/]: ");
        read(&mut operator);
        
        #Cleans the input
        let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();

        #Sets the acceptable answers
        let operators = String::from("+-*/");
        
        #If the answer is not appropriate, it resets the loop
        if !operators.contains(operator) {
            println!("unknown operator");
            continue;
        }
        
        #Does math
        let result = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => panic!("error in operator")
        };
        #Displays the result!
        println!("the result of {} {} {} = {}", num1, operator, num2, result);
    }
}

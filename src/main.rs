use std::io::Write;
enum Operations {
    Add,
    Sub,
    Div, 
    Mult,
}
impl Operations {
    fn _eval(&self, num1: f32, num2: f32) -> f32{
        match self {
            Operations::Add => num1 + num2,
            Operations::Sub => num1 - num2,
            Operations::Mult => num1 * num2,
            Operations::Div => num1 / num2,
        }
    }
    fn _get_op(oper: &String) -> Result<Self, String>{
        match oper.trim() {
            "+" => Ok(Operations::Add),
            "-" => Ok(Operations::Sub),
            "*" | "x" => Ok(Operations::Mult),
            "/" => Ok(Operations::Div),
            &_ => Err("Incorrect operation".to_string()),
        }
    }
    fn _get_string(msg: &str) -> String {
        let mut op: String = String::new();
        std::print!("{}", msg);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut op).expect("Error");
        op.trim().to_string()

    }
}

struct Input;
impl Input {
    fn _parse_data(msg: &str) -> f32 {
        let mut input: String = String::new();
        std::print!("{}", msg);
        std::io::stdout().flush().unwrap();

        std::io::stdin()
                .read_line(&mut input)
                .expect("Couldn't get data");
        input.trim().parse().expect("NaN")
    }
}

fn main(){
    std::println!("Basic Calculation");
        let num11: f32 = Input::_parse_data("Enter 1st number: ");
        let num22: f32 = Input::_parse_data("Enter 2nd number: ");
        let op: String = Operations::_get_string("Enter operation (+ - / *): ");

        std::println!("{num11} {op} {num22} = {}", Operations::_eval( &(Operations::_get_op(&op).unwrap()), num11, num22) );
}

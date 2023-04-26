use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}",input);
    let mut int_input =String::new();
    io::stdin().read_line(&mut int_input).expect("failed to read line");
    let int_input: i64 = int_input.trim().parse().unwrap();
    println!("{}", int_input+2)

}

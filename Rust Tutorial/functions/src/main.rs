fn main() {
    snake_case();
    let result = add_numbers(10,14);
    let number = {
        let x=3;
        x+1
    };
    println!("{}", number);
    println!("{}", result);
}

fn snake_case(){
    println!("test has been called");
}
fn add_numbers(x: i32, y:i32) -> i32{
   println!("the sum is:{}", x+y); 
   let result = x+y;
   if result >10 {
    return result - 9;
   }
   return result;
}

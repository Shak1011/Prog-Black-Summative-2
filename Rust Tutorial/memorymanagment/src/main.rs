fn main() {
    println!("Hello, world!");
}

fn first() {
    let x = 2;
} // x being equal to two is put into ram whilst the function is running
//however once the function is finished it is removed from ram

fn second() {
    let x = 2;
    let y = x;
}//x gets appended into the stack first then y gets appended in with the value stored in x
// y gets popped of the stack first as stack is LIFO data structure

fn string(){
    let string = String::from("hello");
}// string from causes string to be stored on the heap a lot more time consuming as it has to look through the heap
//on the stack string as a variable is stored with the value being the address on the heap


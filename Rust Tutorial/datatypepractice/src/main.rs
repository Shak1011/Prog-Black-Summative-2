fn main() {
    let floating_point =10.92;
    let unsigned: u8 = 255;
    let true_or_false: bool = true;
    let signed: i8 = 127;
    let letter: char='o';
    let mut tup:(i32,bool,char)=(1,true,'s');
    let tup2:(i8,bool,char)=(1,true,'s');
    println!("{}",tup.0);
    tup.0=10;
    println!("{}",tup.0);
    let mut arr= [1,2,3,4,5];
    arr[4] = 3;
    println!("{}",arr[4]);
}

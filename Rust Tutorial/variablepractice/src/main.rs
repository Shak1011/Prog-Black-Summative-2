fn main() {
    const SECONDS_IN_MINUTE: u32=60;
    println!("{}", SECONDS_IN_MINUTE);

    let x=3;
    println!("x is: {}", x);
    {
        let x=x-2;
        println!("x is:{}", x)
    }
    let x=x+10;
    println!("x is: {}", x);
    let x="hello";
    println!("x is: {}", x);
}

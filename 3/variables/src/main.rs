fn main() {
    // let  x = 5;
    // println!("the value of x is: {x}");
    // let x = 19;
    // println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    // let mut spaces = "   ";
    // spaces = spaces.len(); // error
}

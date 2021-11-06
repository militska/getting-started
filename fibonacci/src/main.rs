use std::io;


fn main() {
    println!("Enter number:");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("fail");

    let num: u32 = match num.trim().parse() {
        Ok(num) =>num,
        Err(_) => 0
    }  ;
    fib(num)
}


fn fib(num: u32) {
    for stepper in 0..num {
        if stepper == 0 {
            println!("{}", stepper);
            continue;
        }

        if stepper == 1 {
            println!("{}", stepper);
            continue;
        }
        println!("{}", (stepper - 1) + (stepper - 2));

        if stepper == num {
            break;
        }
    }
}
use std::io;

fn main() {
    loop {
        println!("Change mode: ");
        println!("1. fn->c");
        println!("2. c->fn");

        let mut mode = String::new();

        io::stdin()
            .read_line(&mut mode)
            .expect("fail");

        let mode: u32 = match mode.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Введите температуру");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("fail");

        let temp: i32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if mode == 1 {
            println!("{:.5}", (temp - 32) * 5 / 9);
        } else {
            println!("{:.2}", temp * 9 / 5 + 32);
        }
    }
}

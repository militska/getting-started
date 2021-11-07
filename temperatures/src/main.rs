use std::io;

fn main() {
    loop {
        const F_TO_C: i8 = 1;
        const C_TO_F: i8 = 2;

        println!("______");


        let (mode, res_mode) = enter_mode();
        if !res_mode {
            continue
        }

        let (temp, res_temp) = enter_temp();
        if !res_temp {
            continue
        }
        match mode {
            F_TO_C => {
                println!("Результат: {:.5}", (temp - 32) * 5 / 9);
                break
            },
            C_TO_F => {
                println!("Результат: {:.2}", temp * 9 / 5 + 32);
                break
            },
            _ => continue,
        }
    }
}


fn enter_mode() -> (i8, bool) {
    println!("Change mode: ");
    println!("1. fn->c");
    println!("2. c->fn");

    let mut mode = String::new();
    io::stdin()
        .read_line(&mut mode)
        .expect("fail");

    let mode: i8 = match mode.trim().parse() {
        Ok(num) => num,
        Err(_) => { return (0, false) },
    };

    match mode {
        1..=2 => {},
        _ => {
            println!("Выбран некорректный режим");
            return (0, false)
        }
    }
    return (mode, true)
}


fn enter_temp() -> (i32, bool) {
    println!("Введите температуру");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("fail");

    let temp: i32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => { return (0, false) },
    };

    (temp, true)
}
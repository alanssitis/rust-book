use std::io;

fn main() {
    println!("Get nth fibonacci number!");

    let nth: u32;
    loop {
        println!("Input n.");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        nth = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    let fibonacci = get_nth_fibonacci(nth);
    println!("The {}-th fibonacci number is {}", nth, fibonacci);
}

fn get_nth_fibonacci(nth: u32) -> u32 {
    let mut x = 0;
    let mut y = 1;

    if nth == 0 {
        return x;
    }

    for _ in 1..nth {
        let z = x + y;
        x = y;
        y = z;
    }

    return y;
}

use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    // 生成随机值
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        // 将输入值绑定到guess中
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 类型转换,做容错判断
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        // 比较输入的值和随机值
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too great!"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
        }
    }
}

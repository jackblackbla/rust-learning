use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("숫자를 맞춰보세요!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("숫자를 입력하세요!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("읽어들이는데 실패했습니다.");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("넌 이걸 예측했어 {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("너무 작아!"),
            Ordering::Greater => println!("너무 커!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    }
}
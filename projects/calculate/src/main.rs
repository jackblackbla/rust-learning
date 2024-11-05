fn main() {
    println!("첫 번째 숫자를 입력하세요:");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).unwrap();

    println!("두 번째 숫자를 입력하세요:");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).unwrap();


    let num1: i32 = input1.trim().parse().unwrap();
    let num2: i32 = input2.trim().parse().unwrap();

    println!("어떤 연산을 하시겠습니까? (+,-,*,/)");

    let mut op = String::new();

    std::io::stdin().read_line(&mut op).unwrap();

    let op = op.trim();

    match op {
        "+" => println!("결과: {}", num1 + num2),
        "-" => println!("결과: {}", num1 - num2),
        "*" => println!("결과: {}", num1 * num2),
        "/" => {
            if num2 == 0 {
                println!("0으로 나눌 수 없습니다!");
            }
            else {
                println!("결과: {}", num1 / num2);
            }
        }
        _ => println!("잘못된 연산자입니다."),
    }
}
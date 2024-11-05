use std::fs;

fn find_string_in_text(text: &str, search_word: &str) {
    for (line_number, line) in text.lines().enumerate() {
        if line.contains(search_word) {
            println!("라인 {}: {}", line_number + 1, line);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let contents = fs::read_to_string("input.txt")?;

    let processed_text = process_text(&contents);

    fs::write("output.text", processed_text)?;

    println!("텍스트 처리가 완료되었습니다!");
    Ok(())
}

fn process_text(text: &str) -> String {
    text.lines()
        .enumerate()
        .map(|(i, line)| {

            format!("{}: {}", i + 1, line.to_uppercase())
        })
        .collect::<Vec<_>>()
        .join("\n")
}
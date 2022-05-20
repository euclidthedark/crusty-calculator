use crusty_calculator::tokenizer::tokenize;

fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();
    match tokenize(input) {
        Ok(()) => println!("Computed!"),
        Err(_) => println!("Illegal Tokens.")
    }
}

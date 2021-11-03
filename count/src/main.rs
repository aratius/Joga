use std::io;

// ユーザに予想を入力してもらい、それを出力するコード
fn main() {
    println!("Guess the number");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

}

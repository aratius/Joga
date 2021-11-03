use std::io;  // ユーザ入力を受け付ける能力などの実用的な機能の多くを使用することができる
use rand::Rng;

// ユーザに予想を入力してもらい、それを出力するコード
fn main() {
    println!("Guess the number");

    // 乱数を生成
    // rand::thread_rng -> 乱数生成器を返却
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secrest number is: {}", secret_number);

    println!("Please input your guess");

    // :: -> new が String型の関連関数であることを表す
    // 関連関数　-> 型に対して実装された関数 （static method)
    let mut guess = String::new();

    // stg::io::Stdinオブジェクトを返す -> ターミナルへの標準入力へのハンドル
    // & -> 参照であることを示す
    io::stdin().read_line(&mut guess)
        // Result型を返すメソッド
        // Result型は列挙型（enum） OkかErrをもつ
        // expectはResultオブジェクトがErrだった場合に、プログラムをクラッシュさせ、引数として渡されたメッセージを表示する
        .expect("Failed to read line");

    // println!マクロのプレースホルダー
    println!("You guessed: {}", guess);

}

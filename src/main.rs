use std::io;

fn main() {
    println!("数を当ててごらん");
    println!("ほら、予想を入力してね");

    let mut guess = string::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("行の読み込みに失敗しました");
    println!("次のように予想しました：　{}", guess);
}

// メイン関数
fn main() {
    nlp00();
    nlp01();
}

// 00. 文字列の逆順
fn nlp00() {
    let str1 = "stressed";
    let str2 = str1.chars().rev().collect::<String>();
    println!("{}", str2);
}

// 01. 「パタトクカシーー」
fn nlp01() {
    let str1 = "「パタトクカシーー」";
    let str1len = str1.chars().count();
    let mut number = 0;
    for element in str1.chars() {
        if number < str1len - 1 {
            if number % 2 == 1 {
                println!("{}", element);
            }
        }
        number = number + 1;
    }
}

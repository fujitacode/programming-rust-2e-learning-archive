// コマンドライン引数について

use std::env;
use std::str::FromStr;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);

    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

fn main() {
    let mut values = Vec::new();

    for arg in env::args().skip(1) {
        values.push(
            u64::from_str(&arg)
                .expect("数値への変換に失敗しました"),
        );
    }

    if values.is_empty() {
        eprintln!("使い方: gcd 数値1 数値2 ...");
        std::process::exit(1);
    }

    let mut result = values[0];
    for value in &values[1..] {
        result = gcd(result, *value);
    }

    println!("{:?} の最大公約数は {} です", values, result);
}
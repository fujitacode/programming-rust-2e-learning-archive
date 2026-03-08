
// Euclidean Algorithm
// 2つの整数の最大公約数 を求めるアルゴリズム。

// アルゴリズム概要:
// gcd(a, b) = gcd(b, a % b)

// 例:
// gcd(150, 30)
// → 150 % 30 = 0
// → gcd(30, 0)
// → 30

// 計算量:
// O(log(min(a, b)))

/// 最大公約数 (Greatest Common Divisor) を求める関数
///
/// # Arguments
/// * `n` - 正の整数
/// * `m` - 正の整数
///
/// # Returns
/// * `n` と `m` の最大公約数
///
/// # Panics
/// `n` または `m` が 0 の場合 panic する

fn gcd(mut n: u64, mut m: u64) -> u64 {

    // 0 が入ると gcd の定義が崩れるのでチェック
    assert!(n != 0 && m != 0);

    // m が 0 になるまで繰り返す
    while m != 0 {

        // n <= m の形に整える
        // (小さい方を割る数にする)
        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        // ユークリッドの互除法の核心
        // m = m % n
        m = m % n;
    }

    // 最終的に n が最大公約数になる
    n
}

fn main() {

    // 150 と 30 の最大公約数
    println!("{}", gcd(150, 30));

}
// ユニットテストについて
// コンパイルに含まれずcargo testにのみ実行される便利な機能。

// 主にtestの中で用いられる
// assert!(条件) → 条件が true か見る
// assert_eq!(左, 右) → 左右が等しいか見る
// assert_ne!(左, 右) → 左右が等しくないか見る

#[test]
fn test_hoge() {
    assert_eq!(hoge(150, 30), 30);
    assert_eq!(hoge(18, 33), 3);
    assert_eq!(hoge(14, 21), 7);
}

fn hoge(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn main() {
    println!("{}", hoge(150, 30));
}
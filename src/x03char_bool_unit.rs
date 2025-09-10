use std::mem::size_of_val;

// Char - 01
fn x01_char() {
    let c1 = 'a';

    // 1 (standard) char = 4 bytes
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    // 1 (unique) char = still 4 bytes
    println!("Success!");
}

// Char - 02
fn x02_char() {
    fn print_char(c: &'static str) {
        println!("{}", c);
    }

    // When it's unique char, it's static str
    let c1 = "中";
    print_char(c1);
}

pub fn invoker() {
    println!("===== 03 - Char, Bool, and Unit =====");

    x01_char();
    x02_char();
}

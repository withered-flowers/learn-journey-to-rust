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

// Boolean - 3
fn x03_boolean() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}

// Boolean - 4
fn x04_boolean() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

// Unit Type - 5
fn x05_unit_type() {
    // Make it work, don't modify `implicitly_ret_unit` !
    fn implicitly_ret_unit() {
        println!("I will return a ()");
    }

    // Don't use this one
    fn _explicitly_ret_unit() -> () {
        println!("I will return a ()");
    }

    let v0: () = ();

    let _v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit());

    println!("Success!");
}

// Unit Type = 6
fn x06_unit_type() {
    // Unit = return type void = 0
    let unit: () = ();

    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

pub fn invoker() {
    println!("===== 03 - Char, Bool, and Unit =====");

    x01_char();
    x02_char();
    x03_boolean();
    x04_boolean();
    x05_unit_type();
    x06_unit_type();
}

use std::ops::{Range, RangeInclusive};

// Integer - 01
fn x01_integer() {
    let x: i32 = 5;
    let mut _y = 5;

    _y = x;

    let _z = 10; // Type of z ?

    println!("Success!");
}

// Integer - 02
fn x02_integer() {
    let _v: u16 = 38_u8 as u16;

    println!("Success!");
}

// Integer - 03
fn x03_integer() {
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }

    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Integer - 04
fn x04_integer() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

// Integer - 05
fn x05_integer() {
    let v1 = 247_u8 + 8;
    let v2 = u8::checked_add(119, 8).unwrap();
    println!("{},{}", v1, v2);
}

// Integer - 06
fn x06_integer() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    // 1_024 = 1024
    // 0xff = 255
    // 0o77 = 63
    // 0b1111_1111 = 255
    // 1024 + 255 + 63 + 255 = 1597
    assert!(v == 1597);

    println!("Success!");
}

// Floating Point - 07
fn x07_floating_point() {
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }

    let x = 1_000.000_1; // ?
    let _y: f32 = 0.12; // f32
    let _z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

// Floating Point - 08
fn x08_floating_point() {
    // 0.1, 0.2 and 0.3 will be inferred as f64
    // when 0.1 + 0.2 in f64 will result = 0.30000000000000004
    // not equal to 0.3 f64
    // Solution: back to single precision floating (f32)
    // 0.1 in f32 and 0.2 in f32 will result 0.3 in f32

    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);

    println!("Success!");
}

// Range - 09
fn x09_range() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    // -3 + -2 + -1 + 0 + 1
    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}

// Range - 10
fn x10_range() {
    // 1 2 3 4 (not) 5
    assert_eq!((1..5), Range { start: 1, end: 5 });

    // 1 2 3 4 (with) 5
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

// Computations - 11
fn x11_computations() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6_f32 / 3.2_f32 == 3.0_f32);

    assert!(24 % 5 == 4);

    // // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

pub fn invoker() {
    println!("===== 02 - Numbers =====");
    x01_integer();
    x02_integer();
    x03_integer();
    x04_integer();
    x05_integer();
    x06_integer();
    x07_floating_point();
    x08_floating_point();
    x09_range();
    x10_range();
    x11_computations();
}

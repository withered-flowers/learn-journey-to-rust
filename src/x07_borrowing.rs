// 01 - Borrowing
fn x01_borrowing() {
    let x = 5;

    // Fill the blank
    // let p = __;

    // Use & to reference to memory / pointer
    let p = &x;

    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

// 02 - Borrowing
fn x02_borrowing() {
    let x = 5;
    let y = &x;

    // Modify this line only
    // assert_eq!(5, y);
    // Use *y to reference the pointer value
    assert_eq!(5, *y);

    println!("Success!");
}

// 03 - Borrowing
fn x03_borrowing() {
    #[allow(unused_variables)]
    fn borrow_object(s: &String) {}

    #[allow(unused_mut)]
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

pub fn invoker() {
    println!("\n===== 07 - Reference & Borrowing =====");

    x01_borrowing();
    x02_borrowing();
    x03_borrowing();
}

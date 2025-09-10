// Binding and Mutability - 1
fn x01_binding() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

// Binding and Mutability - 2
fn x02_binding() {
    let mut first_number: i32 = 1;
    first_number += 2;

    assert_eq!(first_number, 3);
    println!("Success!");
}

// Scope - 3
fn x03_scope() {
    let x: i32 = 10;
    let y: i32 = 20;
    {
        let y: i32 = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, y);
}

// Scope - 4
fn x04_scope() {
    fn define_x() -> &'static str {
        let x = "hello";
        x
    }

    println!("{}, world", define_x());
}

// Scope - 5
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn x05_scope() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

// Scope - 6
// Remove a line in the code to make it compile
fn x06_scope() {
    let mut _x: i32 = 1;
    _x = 7;
    // Shadowing and re-binding
    let _x = _x;
    // x += 3;

    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!";

    println!("Success!");
}

// Unused Variable - 7
fn x07_unused_variable() {
    let _x = 1;
}

// Destructuring - 8
fn x08_destructuring() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

// Destructuring Assignments - 9
fn x09_destructuring_assignment() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}

pub fn invoker() {
    println!("===== 01 - Variables =====");
    x01_binding();
    x02_binding();
    x03_scope();
    x04_scope();
    x05_scope();
    x06_scope();
    x07_unused_variable();
    x08_destructuring();
    x09_destructuring_assignment();
}

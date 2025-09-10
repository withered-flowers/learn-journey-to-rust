fn x00_examples() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        // 2 * x; // This will result in unit since there's semicolon
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

// Exercise - 01
fn x01_exercises() {
    let v = {
        let mut _x = 1;
        _x += 2
    };

    assert_eq!(v, ());

    println!("Success!");
}

// Exercise - 02
fn x02_exercises() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert!(v == 3);

    println!("Success!");
}

// Exercise - 03
fn x03_exercises() {
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }

    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}

pub fn invoker() {
    println!("===== 04 - Statements & Expressions =====");

    x00_examples();
    x01_exercises();
    x02_exercises();
    x03_exercises();
}

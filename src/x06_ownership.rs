// 01 - Ownership
fn x01_ownership() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    // let y = x; // This will cause change ownership

    // ownership of string has changed from x to y, so we can't print both
    // println!("{}, {}", x, y);

    // Only can print y
    // println!("{}", y);

    // If we want to print x and y both, we need to clone it
    // Now if we clone, the ownership will be their own (x and y)
    let y = x.clone();

    // Now we can print both
    println!("{}, {}", x, y);
}

// 02 - Ownership
fn x02_ownership() {
    // Only modify the code below!
    fn take_ownership(s: String) -> String {
        println!("{}", s);
        // Returning this value will result of changing ownership
        s
    }

    // Do not modify this code

    // ! Remember: String::from will allocated on heap
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// 03 - Ownership
fn x03_ownership() {
    // Only modify the code below!
    fn give_ownership() -> String {
        let s = String::from("Hello world");

        // Convert String to Vec
        // if we're using into_bytes => Data Type will be Vec<u8>
        // if we're using as_bytes => Data Type will be &[u8]
        // let _s = s.into_bytes();

        // Or for the fastest and efficient way, comment this line
        let _s = s.as_bytes();

        s
    }

    // Do not modify this code
    let s = give_ownership();
    println!("{}", s);
}

// 04 - Ownership
// Fix the error without removing any code
fn x04_ownership() {
    let s = String::from("Hello World");

    // First Solution - use clone
    // print_str(s.clone());

    // Second Solution - use &
    print_str(&s);

    println!("{}", s);

    // If we're using Second Solution, try to use this
    // & = reference to value instead of the value itself
    fn print_str(s: &String) {
        // Instead of this
        // fn print_str(s: String) {
        println!("{}", s)
    }
}

pub fn invoker() {
    println!("===== 06 - Ownership =====");

    x01_ownership();
    x02_ownership();
    x03_ownership();
    x04_ownership();
}

use std::{thread, time};

// 01 - Functions
fn x01_functions() {
    fn sum(x: i32, y: i32) -> i32 {
        return x + y;
    }

    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

// 02 - Functions
fn x02_functions() {
    fn print() -> () {
        println!("Success!");
    }

    print();
}

// 03 - Functions
// Allow unused code and unreachable code (supress warning)
#[allow(dead_code, unreachable_code)]
fn x03_functions() {
    // "!" means it will return "never"
    fn never_return() -> ! {
        // Using this will make the code panic and stopped
        // panic!("I return nothing");

        // A little bit graceful way is to use loop and sleep
        // (But it will still cause infinite loop)
        loop {
            println!("Anything means nothing ...");
            thread::sleep(time::Duration::from_secs(1));
        }
    }

    never_return();

    // This statement will never be reached
    println!("Failed!");
}

// 04 - Diverging Functions
#[allow(dead_code, unreachable_code)]
fn x04_diverging_functions() {
    // Option(al) to returning i32
    fn get_option(tp: u8) -> Option<i32> {
        match tp {
            1 => {
                // TODO
            }
            _ => {
                // TODO
            }
        }

        never_return_fn()
    }

    fn never_return_fn() -> ! {
        // Tell this function is unimplemented yet
        // unimplemented!("Not implemented yet");

        // Make this a panic one
        // panic!("Oh no, this is a panic message");

        // Make this a part of todo to implement later
        todo!("Unfinished yet, TODO later");

        // loop {
        //     thread::sleep(time::Duration::from_secs(1));
        // }
    }

    println!("Success!");
}

// 05 - Diverging Functions
#[allow(dead_code)]
fn x05_diverging_functions() {
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            print!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}

pub fn invoker() {
    println!("===== 05 - Functions =====");

    x01_functions();
    x02_functions();
    // Infinite Loop, so comment this
    // x03_functions();
    x04_diverging_functions();
    // Will trigger panic, so comment this
    // x05_diverging_functions();
}

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

// 05 - Ownership
fn x05_ownership() {
    // Do not use clone, try to use copy instead
    // let x = (1, 2, (), "hello".to_string());
    // let y = x.clone();

    // "hello".to_string() will make a String
    // Since we're owning the x to y below
    // we only need to use reference (&'static str)
    // That means, we don't need to use to_string()
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}

// 06 - Mutability
fn x06_mutability() {
    let s = String::from("Hello ");

    let mut s1 = s;

    // String is not mutable by default, so we need to set it as mut above
    s1.push_str("World!");

    println!("Success!");
}

// 07 - Mutablility
fn x07_mutability() {
    let x = Box::new(5);

    // update this line
    // let ...

    // We can create y from the x and make y own it
    // let mut y = x.to_owned();

    // Or we can make a new Box::new
    // Box::new will allocate memory and the value of it
    let mut y = Box::new(3);

    // So we can use the reference to pointer and set the new value
    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}

// Example - Partial Move
fn x_example_partial_move() {
    /*
     * Within the destructuring of a single variable, both by-move and by-reference pattern bindings can be used at the same time. Doing this will result in a partial move of the variable, which means that parts of the variable will be moved while other parts stay. In such a case, the parent variable cannot be used afterwards as a whole, however the parts that are only referenced (and not moved) can still be used.
     */

    println!("----- Example Partial Move -----");

    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(28),
    };

    // `name` is moved out of variable person, but `age` is referenced
    // "name" is a String = not implement Copy = name is not owned by person
    // "age" is borrowed (referenced), so the owner still person, but it is referenced.
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // since "name" is now owned by the new Person, this will be partially invalid
    // Will cause an error, since "name" has been moved out.
    // "person" no longer have full ownership = error
    // println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);

    println!("----- End of Example Partial Move -----");
}

// 08 - Ownership
fn x08_ownership() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // Modify this line only, don't use `_s`
    // Since _s can't be used, and we can't use t.0 anymore (ownership changed to _s), so we can only use t.1, which is still owned by t
    println!("{:?}", t.1);
}

fn x09_ownership() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    // We can use cloning data (to_owned)
    // let (s1, s2) = t.to_owned();

    // Or we can borrow the entire tuple and then destructure the reference
    // let (s1, s2) = &t;

    // Or we can use the ref to referenced from memory (borrowing)
    // Move the tuple "t" but creates references to its field during destructuring
    // Advantege: Can avoid explicit referencing of the whole tuple.
    let (ref s1, ref s2) = t;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

pub fn invoker() {
    println!("===== 06 - Ownership =====");

    x01_ownership();
    x02_ownership();
    x03_ownership();
    x04_ownership();
    x05_ownership();
    x06_mutability();
    x07_mutability();
    x_example_partial_move();
    x08_ownership();
    x09_ownership();
}

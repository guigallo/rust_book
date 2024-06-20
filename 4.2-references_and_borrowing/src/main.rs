fn main() {
    println!("4.2 References and Ownership");
    references();
    mutable_references();
    mutable_reference_restrictions();
    dangling_references();
    the_rules_of_references();
}

fn references() {
    // we call the action of creating a reference borrowing.

    // a reference is like a pointer in that it's an address we can follow to
    // access the data stored at that address.
    // that data is owned by some other variable.
    // unlike a pointer, a reference is guaranteed to point to a valid value of
    // a particular type for the life of that reference.

    example1();
    fn example1() {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);
        // &s1 is a reference to the value of s1 but does not own it.

        println!("the length of '{}' is {}", s1, len);
    }

    fn calculate_length(s: &String) -> usize {
        // s is a reference to a String
        s.len()
    } // s goes out of scope but does not have ownership of what it refers to.
      // so it is not dropped.

    // attempting to modify a borrowed value
    // fn attempt_modify_borrowed() {
    //     fn main() {
    //         let s = String::from("hello");
    //
    //         change(&s);
    //     }
    //
    //     fn change(some_string: &String) {
    //         some_string.push_str(", world"); // fail here
    //     }
    // }
}

fn mutable_references() {
    // allow attempt_modify_borrowed to modify a borrowed value
    allowed_modify_borrowed();
    fn allowed_modify_borrowed() {
        main();
        fn main() {
            let mut s = String::from("hello"); // mut

            change(&mut s); // '&mut s' is a mutable reference
        }

        fn change(some_string: &mut String) {
            // update function signature '&mut'
            some_string.push_str(", world");
        }
    }
}

fn mutable_reference_restrictions() {
    // mutable references big restriction:
    // a value can have only one mutable reference to it.
    // wrong code below:
    // fn try_create_two_mut_ref() {
    //     let mut s = String::from("Hello");
    //
    //     let r1 = &mut s;
    //     let r2 = &mut s;
    //
    //     println!("{}, {}", r1, r2);
    // }
    //
    // the restriction preventing multiple mutable references to the same data at the same time
    // allows for mutation but in a vary controlled fashion.
    //
    // prevent data races at complie time.
    // a data race is similar to a race condition and happens when these three behaviors occur:
    //   - two or more pointers access the same data at the same time.
    //   - at least one of the pointers is being used to write to the data.
    //   - there's no mechanims being used to synchronize access to the data.
    // data races cause undefined behavior and can be difficult to diagnose and fix when you're
    // trying to track them down at runtime; Rust prevents this problem by refusing to compile code
    // with data races;

    // allowing for multiple mutable references, just not simultaneous ones:
    let mut s = String::from("Hello");

    {
        let _r1 = &mut s;
    } // _r1 goes out of scope here.

    let _r2 = &mut s;

    // Rust enforces a similar rule for combining mutable and immutable references
    // {
    //     let mut s = String::from("Hello");
    //
    //     let _r1 = s;
    //     let _r2 = s;
    //     let _r2 = &mut s;
    // }
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s;
    println!("{}", r3);
}

fn dangling_references() {
    // Is a pointer that references a location in memory that may have been
    // given to someone else
    
    // In languages with pointer, it's easy to erroneuosly create a dongling
    // pointer by freeing some memory while preserving a pointer to that memory

    // Rust compiler garanttes that references will never be dangling referens:
    // if you hve a reference to some data, the compiler will ensure that the
    // data will not go out of scope before the reference to the data does

    fn main() {
        #[allow(unused_variables)]
        let reference_to_nothing = dangle();
        #[allow(unused_variables)]
        let reference_to_something = no_dangle();
    }
    
    // this function's return type contains a borrowed value, but there is no value
    // for it to be borrowed from.
    fn dangle() -> &String { // dangle returns a reference to a String
        let s = String::from("hello"); // s is a new String

        &s // we returna reference to the String, s
    } // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

    fn no_dangle() -> String {
        let s = String::from("hello");
        s
    }
}

fn the_rules_of_references() {
    // - At any given time, you can have either one mutable reference or any
    // number of immutable references.
    // - References must always be valid.
}


fn main() {
    println!("4.1 Understanding Ownership");

    ownership();
    the_stack_and_the_heap();
    ownership_rules();
    variable_scope();

    string_type();
    memory_and_allocation();
    variables_and_data_interacting_with_move();
    variables_and_data_interacting_with_clone();
    stack_only_data_copy();

    ownership_and_functions();
    return_value_and_scope();
}

fn ownership() {
    // most unique feature and has deep implications for te rest of the language.
    // memory is managed through a system of ownership with a set of rules that
    // the compiler checks at compile time.
    // make memory safety guarantees without needing a garbage collector.

    // # what is ownership?
    //
    // some languages have garbage collection that regularly looks for
    // no-longer-used memory.
    // in rust memory is managed through a system of ownership with a set of
    // rules that the compiler checks at compile time.
    // if any of the ownership rules are violated, your program will not compile.
    // none of the ownership features slow down your program while it's running.
}

fn the_stack_and_the_heap() {
    // # the stack and the heap
    //
    // not common to worry about in others language.
    // in rust affects the aproach to deal with values.
    // both are parts of memory availablle to use at runtime, but there's differences.

    // ## stack
    //
    // LIFO (last in, first out).
    // data can be pushed and popped of the stack.
    // must have a known, fixed size.
    // is faster, because dont neet to allocate, is always at the top of the stack.
    //
    // ## heap
    //
    // less organized.
    // to put data on heap, you request a space.
    // the memory allocator finds and empty spot, and returns a pointer.
    // this process is called allocating on the heap, or just allocating.
    // accessing data is slower, because have to follow the pointer.
    // processor are faster if they jump around less in memory.

    // ##
    //
    // When your code calls a function, the values passed into the function
    // (including, potentially, pointers to data on the heap) and the function’s
    // local variables get pushed onto the stack. When the function is over,
    // those values get popped off the stack.
    // Keeping track of what parts of code are using what data on the heap,
    // minimizing the amount of duplicate data on the heap, and cleaning up
    // unused data on the heap so you don’t run out of space are all problems
    // that ownership addresses.
}

fn ownership_rules() {
    // - each value in rust has an owner.
    // - there can only be one owner at a time.
    // - when the owner goes out of scope, the value will be dropped.
}

fn variable_scope() {
    // s is not valid here, it's not yet declared.
    let _s = "hello"; // s is valid from this point forward.

    // do stuff with s.
} // this cope is now over, and s is no longer valid.

fn string_type() {
    // to illustrate we need a data type that is more complex.
    // types covered previously are of a known size, can be store on the stack.
    // store on the heap and explor how rust knows when to clean up that data.

    // we'll concentrate on the parts os string that relate to ownership.
    // these aspects also aplly to other complex data types.
    //
    // String is stored on heap

    let mut s = String::from("hello"); // :: allows us to namespace this particular from
    s.push_str(", world!"); // push_str() appends a literal to s String

    println!("{}", s); // this will print `hello, world!`
}

fn memory_and_allocation() {
    // str literal, we know the contents at compile time, so the text is hardcoded
    // directly into the final executable.
    // is fast and efficient.

    // String:
    //
    // - the memory must be requested from the memory allocator at runtime.
    //   is done by us. ex. String::from
    //   pretty much universal in programming languages.
    //
    // - we need a way of returning this memory to the allocator when we're
    //   in languages with a GC, it keeps track of and cleans up memory that
    //   isn't being used anymore.
    //   in most languages without GC, it's our reponsibility to identify when
    //   memory is no longer being used and to call code to explicitly free it.
    //
    // - rust takes a different path:
    //   the memory is automatically returned once the variable that owns it goes
    //   out of scope. ex. string_type()
    //   this function is called `drop`, and it's where String can put the code to
    //   return the memory.
    //   rust calls drop automatically at the closing curly bracket.

    // in c++, the pattern of deallocating resource at the end of an item's
    // lifetime is sometimes called Resource Acquisition Is Initialization (RAII).
    // the drop function in rust will be familiar to you if you' ve used RAII patterns.
    //
    // this pattern has a profound impact on the way rust code is writeen.
}

fn variables_and_data_interacting_with_move() {
    // multiple variables can interact with the same data;
    let x = 5;
    let _y = x;
    // these two 5 values are pushed onto the stack.

    // String version:
    let s1 = String::from("hello");
    // String is made up of three parts:
    // this group is stored on the stack.
    // - pointer: to the memory that hold the contents of the string.
    // - length: how much memory, in bytes, are currently using.
    // - capacity: total amount of memory, in bytes has recived from the allocator.
    // and the memory on head that holds the contentsr.
    let _s2 = s1;
    // this means we copy the pointer, length and capacity that are on the stack.
    // we do not copy the data on the heap that the pointer refers to.

    // as rust automatically calls the drop frunction and both data pointers pointing
    // to the same locations, there is a problem:
    // when s2 and s1 go out of scope, they will both try to free the same memory.
    // this is known as a `double free` error.
    // freeing memory twice can lead to memory corruption, which can potentially lead
    // to secuity vulnerabilities.
    // println!("{}, world", s1); // error[E0382]: borrow of moved value: `s1`
    // there are terms `shallow copy` and `deep copy` while working with other languages.
    // rust also invalidates the first variable, instead of being called a shallow
    // copy, it's known as a `move`. s1 was moved into s2.
}

fn variables_and_data_interacting_with_clone() {
    // to deeply copy the heap data, not just the stack data we can use a common
    // method called `clone`.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn stack_only_data_copy() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // this code sems to contradict what we just learned.
    // the reason is that types such as integers that have a known size at
    // compile time are stored entirely on the stack.
    // there's no difference between depp and shallow copying here, so calling
    // cone wouldn't do anything different from the usual shallow copying.

    // rust wont't let us annotate a type with `Copy` if the type, of any of
    // its parts, has implemented the `Drop` trait.
    // to learn about how to add the `Copy` annotation to your type to implement
    // the trait, see "Derivable Traits" in Appendix C.

    // some types that implements the `Copy` trait:
    // - all the integer types, such as u32.
    // - the Boolean type, bool, with values true and false.
    // - all the floatring-point types, such as f64.
    // - the character type, char.
    // - tuples, if they only containt types that also implment `Copy`.
    //   (i32, i32) implements
    //   (i32, String) do not implements
}

fn ownership_and_functions() {
    // the mechanics of passing a value to a function are similar to those when
    // assigning a value to a variable.
    // passing a variable to a function will move or copy, just as assignment does.

    fn main() {
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's values moves into the function
                            // and so is no longer valid here.

        let x = 5; // x comes into scope.

        makes_copy(x) // x would move into the function,
                      // but i32 is Copy, so it's okay to still use x afterward.
    } // here, x goes out of scope, then s.
      // but because s's values was moved, nothing special happens.

    fn takes_ownership(some_string: String) {
        // some_string comes into scope.
        println!("{}", some_string);
    } // here, some_string goes out of scope and `drop` is called.
      // the backing memory is freed.

    fn makes_copy(some_integer: i32) {
        // some_integer comes into scope.
        println!("{}", some_integer);
    } // here, some_integer goes out of scope. nothing special happens.

    // if we tried to use s after the call to takes_ownership, rust would
    // throw a compile-time error.
    // these static checks protect us from mistakes.

    main();
}

fn return_value_and_scope() {
    // returning values can also transfer ownership.

    fn main() {
        let _s1 = gives_ownership(); // gives_ownership moves its return values into s1.

        let s2 = String::from("hello"); // s2 comes into scope.

        let _s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back,
                                            // which also moves its return value into s3.
    } // here, s3 goes out of scope and is dropped.
      // s2 was moved, so nothing happens.
      // s1 goes out of scope and is dropped.

    fn gives_ownership() -> String {
        // gives_ownership will move its return value
        // into the function that calls it.
        let some_string = String::from("yours"); // some_string comes into scope.

        some_string // some_string is returned and moves out to the calling function.
    }

    // this function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string comes into scope.
        a_string // a_string is returned and moves out to the calling function.
    }

    main();

    // the ownership of a variable follows the same pattern every time:
    // assigning a value to another variable moves it. when a variable that inclues
    // data on the heap goes out of scope, the value will be cleaned up by `drop`
    // unless ownership of the data has been moved to another variable.

    // what if we want to let a function use a value but not take ownership?
    // it's quite annoying that anything we pass in also needs to be passed back
    // if we want to use it again, in addition to any data resulting from the body of
    // the function that we might want to return as well.

    fn other_main() {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("the length of '{}' is {}.", s2, len);
    }

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() return the length of a String

        (s, length)
    }

    other_main();
    // but this is too much ceremony and a lot of work for a concept that
    // should be common. rust has a feature for using a value without
    // transferring ownership, called `references`.
}

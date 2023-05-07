fn main() {
    println!("\n# 3.1 Variables and Mutability");
    variables();
    constants();
    shadowing();

    println!("\n# 3.2 Data types");
    // Rust is statically typed; it must know the types of all variables at compile time
    // Every value is of a certain data type and can be infered by compiler
    // This is for two data type subsets: scalar and compound
    scalar_types();
    compound_types();

    println!("\n# 3.3 Functions");
    functions();
    statements_and_expressions();
    functions_with_return();

    println!("\n# 3.4 Comments");
    // this is a comment

    println!("\n# 3.5 Control Flow");
    if_expression();
    if_in_let_statement();
    repetition_with_loops();
    loop_labels();
    loops_while();
    for_loop();
}

fn variables() {
    println!("\n## Variables");

    // By default variable ar immutable.

    // this wont compile because is trying to change value from immutable variable.
    // let x = 5;
    // x = 6;

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn constants() {
    println!("\n## Constants");

    // constants are ALWAYS immutable.
    // can be declared in any scope.
    // can't be declared with a value that could be computed at runtime.

    println!("Three hours in second is {THREE_HOURS_IN_SECONDS}");
}

fn shadowing() {
    println!("\n## Shadowing");

    // declare a new variable with same name.
    // the last declared variable is what the compiler will see.

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn scalar_types() {
    println!("\n## Scalar");
    // represents a single value.

    println!("\n### Integers");
    // an integer is a number without a fractional component.
    // each variant can be signed or unsigned.

    // Length   Signed   Unsigned
    // 8-bit    i8       u8
    // 16-bit   i16      u16
    // 32-bit   i32      u32
    // 64-bit   i64      u64
    // 128-bit  i128     u128
    // arch     isize    usize

    // can store numbers: (where n means to the power of n)
    // signed: -(2n - 1) to 2n - 1 - 1 inclusive
    // unsigned: 0 to 2n - 1

    // arch depend on the architecture of the computer your program is running on.

    // Number literals can:
    // - multiple numeric types allow a type suffix
    // - can also use _ as a visual separator to easy read. example 1_000 same as 1000

    // Number literals    Example
    // Decimal            98_222
    // Hex                0xff
    // Octal              0o77
    // Binary             0b1111_0000
    // Byte (u8 only)     b'A'

    // The primary situation in which you'd use isize or usize is when indexing some sort of
    // collection.

    // Integer Overflow is when you try to attribute a value bigger than variable size.
    // example 256 value to u8 type
    //
    // to explicitly handle the possibility of overflow, you can use these families of methods
    // provided by the standar library for primitive numeric types:
    // - wrap in all modes with the wrapping_* methods, such as wrapping_add.
    // - return the None value if there is overflow with the checked_* methods.
    // - return the value and a boolean indicating whether there was overflow with the overflow_*
    // methods.
    // - saturate at the value's minimum or maximum values with the saturating_* methods.

    println!("\n#### Floating-point numbers");
    // an integer is a number with a fractional component.
    // the default type if f64 because on modern CPUs, it's roughly the same speed as f32 but is
    // capable of mor precision.
    // all floating-point types are signed.
    let _x = 2.0; // f64 double precision float
    let _y: f32 = 3.0; // f32 single precision float

    println!("\n### Numeric operations");
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // results in -1
    let _remainder = 43 % 5;

    println!("\n## Booleans");
    // true/false.
    // booleans are one byte in size.
    let _t = true;
    let _f: bool = false; // explicit type annotation

    println!("\n## Character");
    // most primitive alphabetic type.
    // should be used with single quotes.
    // is 4 bytes in size and represents a Unicode Scalar Value, wich represent a lot more than
    // just ASCII. Accented letters, Chinese, Japanese, Korean, emoji and zero-width.
    let _c = 'z';
    let _z: char = 'ℤ';
    let _heart_eyed_cat = '😻';
}

fn compound_types() {
    println!("\n## Compound");
    // has two primitive compound types: tuples and arrays.

    println!("\n### Tuple type");
    // is a general way of grouping together a number of values witha a variety of types into
    // one.
    // compound type.
    // have a fixed length.
    // tuple without any values has a special name, unit. expressions implicitly return the unit
    // value if they don't return a ny other value
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, _y, _z) = tup;
    println!("The value of x is: {x}");
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    println!("\n### Array");
    // every element of an array must have the same type.
    // have a fixed length at compile time.
    // a vector is a similar collection type that is allowed to grow or shrink in size.
    //
    // # stack memory:
    // - is a region of memory that is used for local variables and function calls.
    // - accesed more quickly.
    // - array is allocated here.
    // - is managed by the operating system and is organized as a stack data structure, which means
    // that the last item pushed onto the stack is the first item to be popped off.
    // - when a function is called, the arguments and local variables for that function are pushed
    // onto the stack, and the function returns, those variables are popeed of the stack.
    // - this makes stack allocation and deallocation very fast, since it is a simple and efficient
    // memory manegement strategy.
    //
    // # heap memory:
    // - is a region of memory used for dynamic memory allocation.
    // - is managed by the program itself.
    // - when a program need to allocate memory for a data structure whose size is not known at
    // compile time (such as vector), it requests memory from the operating system's heap.
    // - this memory must be explicitly deallocated by the program when it is no longer needed.
    // - since heap allocation and deallocation are more complex operations than stack process,
    // they are generally slower and more error-prone.
    //
    // # stack overflow error:
    // - occurs when a program tries to allocate more memory on the stach than is available.
    // - since stack is a finite region of memory, there is a limit to how much memory can be
    // allocated on the stack.
    // - if a function calls itself recursively too many times, for example, it may eventually run
    // out of stack space and cause a stack overflow error.
    // - when this happens, the program will usually crash or terminate with an error message:
    // _stack_overflow_error();

    let a = [1, 2, 3, 4, 5];
    let _a2: [i32; 5] = [1, 2, 3, 4, 5];
    let _a3 = [3; 5]; // [3, 3, 3, 3, 3];

    // # accessing array elements
    let _first = a[0];
    let _second = a[1];

    // # invalid array element access
    // this is an example of rust's memory safety principles in action.
    // in many low-level languages, this kind of check is not done, and when you provide an
    // incorrect index, invalid memory can be accesed.
    // rust protects you against this kind of error by immediately exiting instead of allowing the
    // memory access and continuing.
    //
    // let _element = a[10]; // throw index out of bounds: the length is 5 but the index is 10
}

fn _stack_overflow_error() {
    fn factorial(n: u32) -> u32 {
        if n == 0 {
            1
        } else {
            n * factorial(n - 1)
        }
    }

    factorial(10_000);
}

fn functions() {
    println!("\n## Functions");
    // fn keyword allow to declare new functions.
    // one of the most important functions in the ruest: the main function, which is the entry
    // point of many programs.
    // rust uses snake case as the conventional style for function and variable names.
    // ruest doesn't care where you define your function, only that they're defined somewhere in a
    // scope that can be seen by the caller.

    // # Parameters
    // special variables that are part of a function's signature.
    // technically, the concrete values are called arguments, but in casual conversation, people
    // tend to use the words parameter and argument intechangeably for either the variables in a
    // function's definition.
    // you MUST declare the type of each parameter.
    fn another_function(x: i32) {
        println!("The value of x is: {x}");
    }
    another_function(5);
    fn print_labeled_measurement(value: i32, unit_lable: char) {
        println!("The measurement is: {value}{unit_lable}");
    }
    print_labeled_measurement(5, 'h');
}

fn statements_and_expressions() {
    println!("\n## Statements and Expressions");
    // function bodies are made up of a series of statements optionally ending in an expression.
    // rust is and expression-based language.

    // # statements: are intructions that perform some action and do not return a value;
    // function definitions are also statements.
    // creating a variable and assigning a value to it is a statement:
    let _y = 6;
    // let _x = (let y = 6); // you can't assign a let statement to anoter variable.
    // in other languages, you can write `x = y = 6` and have both variables have value 6.

    // # expressions: evaluate to a resultant value.
    // consider a math operation, such as 5 + 6, wich is an expression that evaluates to the value
    // 11.
    // expressions can be part of statements.
    // calling a function is an expression.
    // calling a macro is an expression
    // a new scope block created with curly brackets is an expression:
    let z = {
        let x = 3;
        x + 1
    };
    println!("The value of z is: {z}");
    // the x + 1 line doesn't have a semicolon athe the end.
    // expressions do not include ending semicolons, if you add a semicolon to the end of an
    // expression, you turn it into a statement, and it will then not return a vlue.
}

fn functions_with_return() {
    println!("\n## Functions with return");
    // functions can return values to the code that calls them.
    // we don't name return values, but we must declare their type after and arrow ->
    // return value of the function is synonymous with the value of the final expression in the
    // block of the body.
    // can return early by using return and specifying a value.

    fn _five_1() -> i32 {
        5 // if put semicolon here, will change it from an expression to a statement, and get and
          // error
    }
    // is same as:
    fn _five_2() -> i32 {
        let x: i32 = 5;
        return x;
    }
}

fn if_expression() {
    println!("\n## If expression");
    // if this condition is met, run this block of code, if not, run this block of code.
    // condition MUST be a bool.
    //
    // all if expression startch with if, followed by a condition.
    // optionally, we can also include an else expression.
    // can use multiple condition by combining if and else in an else if expression.
    // rust only executes the block for the first true condition, and once it finds one, it doesn't
    // even check the rest.
    //
    // using too many else if expressions can clutter your code, so if you have more than one, you
    // might want to refactor your code to use match for these cases.
    //
    // blocks of code assiciated with the conditions in if expressions are sometime called arms,
    // just like the arms in match expressions.

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }
}

fn if_in_let_statement() {
    println!("\n## If in let statement");
    // because if is an expression, we can use it on the right side of a let statement to assign
    // the outcome to a variable.
    // the values that have the potential to be results from each arm of the if must be the same
    // type.

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("the value of number is: {number}");
}

fn repetition_with_loops() {
    println!("\n## Repetition with loops");
    // loop keyword tells rust to execute a block of code over and over again forever or until you
    // explicitly tell it to stop.
    //
    // break keyword within the loop to tell the program when to stop executing the loop.
    // continue keyword to tell the program to skip over any remaining code in this iteration.
    // can return a value from a loop adding the value after the break expression

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is {result}");
}

fn loop_labels() {
    println!("\n## Loop lables to disambiguate between multiple loops");
    // loops within loops, break and continue apply to the innermost loop at that point.
    // optionally specify a loop label, that must begin with a single quote.

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("end count = {count}")
}

fn loops_while() {
    println!("\n## Conditional loops with while");

    // while the condition is true, the loop runs.
    // it's possible to implement behavior like this using a combination of loop, if, else, and
    // break.
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // loop over the elements of a collection, such as an array.
    // however, this approach is error prone; we could cause the program to panic if the index
    // value or test condition is incorrect; as a more concise alternative, you can use a for
    // loop.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn for_loop() {
    println!("\n## For loop");

    // the safety and conciseness of for loops make them the most commonly used loop construct in
    // rust.

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}

const CONSTVAR: u32 = 60 * 60 *3;
static HELLO_WORLD: &str = "Hello, world!";


fn main() {
    println!("Rust Abridged - 3-Common-Programming-Concepts");
    println!("Constants: Cannot be muted. Stored directly into program's binary. Value can be evaluated at compile time.");
    println!("constvar: {}, staticvar: {}", CONSTVAR, HELLO_WORLD);
    /* shadowing */
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        /* the shadow inside this scope will not be relevant after the scope is over, outside, the
         * value of will be 6.*/
    }
    println!("The value of x is: {x}");

    /* Data types */
    // boolean
    let t = true;
    let f: bool = false;
    println!("t: {}, f: {}", t, f);

    // compund types
    // tuple
    let tup: (i32, f64, u8) = (34, 34.45, 98);
    println!("tup: {:?}", tup);

    testing_functions();
    testing_functions_with_arg(12, true, 'A', 45.67);
    println!("Function with return type: {}", function_with_no_arg_string_return());

    /* Control Flow */
    let a: i32 = 1;
    if a > 2 {
        println!("Number greater than 2");
    }
    else { 
        println!("Number less than equal to 2");
    }
    /* Repetition with loops */
    /*
     * 3 kinds of loops - loop, for, while
     * break and continue as usual
     * */
    /* using loops as expressions */
    let mut counter = 0;
    let x = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    }; /* a semi colon is needed here since we're using loops as expression */
    println!("loop return x: {}", x);
    /* labelling a loop, much like goto labels in C */
    'outer: loop {
        'inner: loop {
            println!("Inner loop");
            break 'inner;
        }
        println!("Outer loop");
        break 'outer;
    }
    /* while loop */
    while counter < 100 {
        counter += 1;
    }
    print_i32(counter);
    /* iterate over an array using loop */
    let a = [1,2,3,4,5]; /* we don't explicitly use a type here because Rust will infer it to be a i32 array */
    for element in a {
        println!("elem arr: {}", element);
    }
    /* iterate within a range using for loop */
    for element in (10..15) {
        println!("elem range: {}", element);
    }
    /* infinite loop */
    /*
    loop {
        println!("Hello World in Rust!");
    }*/
}

fn print_i32(x: i32) ->() {
  println!("i32 value: {}", x);
}

fn testing_functions() -> () {
    println!("Testing function with no arguments");
}

fn testing_functions_with_arg(arg1: i32, arg2: bool, arg3: char, arg4: f64) {
    println!("Testing functions with arguments");
    println!("arg1: {}, arg2: {}, arg3: {}, arg4: {}", arg1, arg2, arg3, arg4);
}

fn function_with_no_arg_string_return() -> String {
    return "Hello world!".to_string(); // convert &str, a string slice to String, a vector of characters
}

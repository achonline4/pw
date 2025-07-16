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
}

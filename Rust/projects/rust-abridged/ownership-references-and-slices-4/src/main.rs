fn main() {
    println!("Ownership");
    let outer_string = transfer_ownership();
    println!("outer string: {}", outer_string);

    let new_heap_mem = deep_copy_data(outer_string);
    println!("new string content: {}", new_heap_mem);

    println!("string slice 1-4 {} from string {}", slice(&new_heap_mem, 1, 4), new_heap_mem);
}

/* Return a slice of string between given index */
fn slice(s: &String, start: usize, end: usize) -> &str {
    use std::ops::Range;
    let r: Range<usize> = start..end;
    /* Both of the following formats are OK */
    //let slice: &str = &s[1..4];
    let slice = &s[r];
    slice
}

fn deep_copy_data(s: String) -> String {
    println!("Input for deep copy: {}", s);
    let newstr = s.clone(); // create a new instance on the heap and copy all data to it.
    return newstr;
}

fn transfer_ownership() -> String {
    let inner_string = String::from("Hello World!"); /* This instance is allocated on heap hence the ownership can be moved */
    inner_string
    /* the following are all correct return statements - 
     * inner_string
     * return inner_string
     * return inner_string; 
     *
     * The following return type is wrong - 
     * inner_string; */
}

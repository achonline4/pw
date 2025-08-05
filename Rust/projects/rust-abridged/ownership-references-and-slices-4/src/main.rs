fn main() {
    println!("Ownership");
    let outer_string = transfer_ownership();
    println!("outer string: {}", outer_string);

    let new_heap_mem = deep_copy_data(outer_string);
    println!("new string content: {}", new_heap_mem);
}

fn deep_copy_data(s: String) -> String {
    println!("Input for deep copy: {}", s);
    let newstr = s.clone();
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

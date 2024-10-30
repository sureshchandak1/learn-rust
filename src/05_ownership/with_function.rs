fn main() {

    let x: u8 = 5;
    print_int(x);
    println!("Value: {}", x);

    let str: String = String::from("Hello"); // str is the owner of Hello
    print_string(str); // transfer of ownership (remove str)
//    println!("str: {}", str); // getting error use str

    let s1: String = String::from("hello"); // s1 owner
    //let len: usize = calculate_lenght(s1); // ownership transfer

    let len1: usize = lenght(s1.clone());
    println!("The length of {} is {}", s1, len1);

    let (s2, len2) = calculate_len(s1);
    println!("The length of {} is {}", s2, len2);

    
}

fn print_int(val: u8) {
    println!("Value: {}", val);
}

fn print_string(item: String) { // Hello - new owner is item
    println!("str: {}", item);
}

fn calculate_lenght(s: String) -> usize { // s will be the new owner
    return s.len();
}

fn calculate_len(s: String) -> (String, usize) { // s will be the new owner
    let length = s.len();
    return (s, length); // return ownership transfer, length
}

fn lenght(s: String) -> usize {
    return s.len();
}

fn main() {

    let x: u8 = 5;
    print_int(x);
    println!("Value: {}", x);

    let str: String = String::from("Hello"); // str is the owner of Hello
    print_string(str); // transfer of ownership (remove str)
//    println!("str: {}", str); // getting error use str

}

fn print_int(val: u8) {
    println!("Value: {}", val);
}

fn print_string(item: String) { // Hello - new owner is item
    println!("str: {}", item);
}
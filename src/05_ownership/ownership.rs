fn main() {
    
    let str1 = String::from("Hello"); // str1 is the owner of Hello
    /* 
        In Rust, when ownership of a value is transferred to another variable, 
        the original variable becomes invalidated.
    */
    let str2 = str1; // transfer of ownership. str2 is the new owner of Hello (Remove str1)

    //println!("str1 = {}", str1);
    println!("str2 = {}", str2);
}
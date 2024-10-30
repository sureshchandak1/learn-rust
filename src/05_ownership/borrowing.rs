fn main() {

    let str: String = String::from("Hello");
    let len = calculate_lenght(&str); // send str address to function (borrow operation)
    println!("The length of {} is {}", str, len);

    let mut s1: String = String::from("Hello ");
    append_string(&mut s1); // allow to change value
    println!("The new string is {} ", s1);

    let mut str1: String = String::from("Hello ");
    let r1 = &str1;
    let r2 = &str1;
    println!("r1 = {}, r2 = {}", r1, r2);

    let w1 = &mut str1;
    w1.push_str("World ");
    println!("w1 = {}", w1);
    let w2 = &mut str1;
    w2.push_str("borrow");
    println!("w2 = {}", w2);

    //println!("w1 = {}, w2 = {}", w1, w2);
}

fn calculate_lenght(s: &String) -> usize { 
    return s.len();
}

fn append_string(s3: &mut String) {
    s3.push_str("World");
}
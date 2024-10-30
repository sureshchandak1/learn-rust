fn main() {

    let emp_info: (&str, u8) = ("Ramesh", 40);

    let emp_name = emp_info.0;
    let emp_age = emp_info.1;

    println!("Name: {}, Age: {}", emp_name, emp_age);

    // destructuring
    let (name, age) = emp_info;
    println!("Name: {}, Age: {}", name, age);
}
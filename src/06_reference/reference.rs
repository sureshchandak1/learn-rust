fn main() {
    let x = 5;
    println!("x = {:p}", &x);
    let y = &x; // y is reference to the value of x, value of x is 5

    println!("y = {:p}", y);
    println!("y = {}", y); // auto de-referencing 

    let mut a = 5;
    a = a + 1;
    let b = &mut a;
    *b = *b + 1; // dereferencing (reach to value)
    println!("a = {}", b);
}

fn calculate_length(s: &String) -> usize {
    return (*s).len(); // *s = dereferencing
}
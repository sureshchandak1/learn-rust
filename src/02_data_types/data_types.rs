fn main() {
    let num1: u8 = 5; // default immutable variable (not allow to re-assign)
    let num2 = 10;
    println!("This value stored in num1 is {}", num1);
    println!("This value stored in num2 is {}", num2);

    let num3: i8 = 100;

    let mut num4: i16 = 200; // allow to reassign (Mutable variable)
    println!("This value stored in num4 is {}", num4);
    num4 = 300;
    println!("This value stored in num4 is {}", num4);

    let string_literal: &str = "Rakesh"; // default is &str type (Fixed Length Strings) - Read only data
    println!("String literal: {}", string_literal);

    let name: String = String::from("Mohan Soni"); // Dynamic Length String, allow to modify string - Heap Allocated
    println!("Name: {}", name);

}
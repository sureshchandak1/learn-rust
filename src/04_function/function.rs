fn main() {

    print("Hello world");
    print("Ramesh");

    println!("{}", add(10, 20));
}

fn print(message: &str) {
    println!("{}", message);
}

fn add(v1: i16, v2: i16) -> i16 {
    return v1 + v2;
}
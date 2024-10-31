fn main() {

    let reference_to_nothing = create_string_ref();
}

fn create_string_ref() -> &String {
    let s: String = String::from("Hello");
    return &s;
    // s is local variable and this variable memory clear when function complete
    // so after completing function s not reference anything
}
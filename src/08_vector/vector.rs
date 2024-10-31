fn main() {

    let mut v: Vec<i32> = Vec::new(); // declaration
    let mut v1 = Vec::<i32>::new(); // declaration

    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v);

    v1.push(10);
    v1.push(20);
    v1.push(30);

    println!("{:?}", v1);

    let mut v2 = vec![40, 50, 60];
    v2.push(70);
    println!("{:?}", v2);
    v2.pop();
    println!("{:?}", v2);

    let mut v3: Vec<&str> = vec!["Hello", "World", "Coders"]; // v3 owner of this array
    //write_vec(v3); // v3 ownership transferred
    write_vec_ref(&mut v3);
    println!("{:?}", v3);
}

fn write_vec(v: Vec<&str>) { // v is the current owner
    println!("{:?}", v);
}

fn write_vec_ref(v: &mut Vec<&str>) { // v is hold referance of vector
    v.push("Rakesh");
    println!("{:?}", v);
}
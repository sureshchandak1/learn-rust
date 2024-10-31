fn main() {

    let mut arr1: [u8;5] = [1, 0, 0, 0, 0]; // array declaration

    println!("{:?}", arr1);

    arr1[2] = 30;
    println!("{:?}", arr1);

    println!("Length: {}", arr1.len());

    let mut arr2: [&str; 3] = ["Hello", "World", "Coders"];
    println!("{:?}", arr2);

    arr2[1] = "Rakesh";
    println!("{:?}", arr2);

    const LEN: usize = 2;
    let arr3: [i8; LEN] = [7, 9];
    println!("{:?}", arr3);

    let mut arr4: [&str; 3] = ["Hello", "World", "Coders"];
    write_arr(arr4);
    println!("{:?}", arr4);
    write_arr_ref(&mut arr4);
    read_arr(&arr4);
}

fn write_arr(mut arr: [&str; 3]) { // new copy of arr
    arr[0] = "Fellow";
    println!("{:?}", arr);
}

fn write_arr_ref(arr: &mut [&str; 3]) { // not new copy of arr
    arr[0] = "Fellow";
    println!("{:?}", arr);
}

fn read_arr(arr: &[&str; 3]) { 
    println!("{:?}", arr);
}
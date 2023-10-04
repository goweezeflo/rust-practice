fn main() {
    let array = [1, 2, 3, 4];

    let slice1 = &array[0..2];
    println!("slice 1 -> {:?}", slice1);

    let slice2 = &array[2..4];
    println!("slice 2 -> {:?}", slice2);
}

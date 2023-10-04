fn main() {
    println!("Basic [for] loop");
    for i in 0..4 {
        println!("{}", i);
    }

    println!("Looping over array items");
    let collection = [1, 3, 5, 7];
    for item in collection {
        println!("{}", item);
    }
}

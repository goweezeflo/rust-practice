fn main() {
    let array = [1, 2, 3];

    println!("Pretty print an array");
    println!("{:?}", array);

    println!("Pretty print an array (formatted)");
    println!("{:#?}", array);

    println!("Print the first element of an array");
    println!("{}", array[0]);

    println!("Print the last element of an array");
    println!("{}", array[array.len()-1]);
}

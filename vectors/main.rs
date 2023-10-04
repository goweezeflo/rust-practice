fn main() {
    // Vector [values] needs to be mutable
    let mut vector = vec![1, 2];
    let mut sum = 0;

    // Increase the size of vector
    vector.push(3);
    vector.push(4);

    // Remove the last element from the vector
    println!("Last element: {:?}", vector.pop());

    for element in vector {
        sum = add(sum, element);
    }

    println!("sum = {}", sum);
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

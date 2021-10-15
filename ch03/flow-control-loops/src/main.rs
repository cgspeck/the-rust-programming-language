fn main() {
    let mut counter = 0;

    // you can return data from a loop...
    let result = loop {
        counter += 1;

        if counter == 10 {
            // with the break keyword
            break counter * 2;
        }
    };

    println!("The counter is {}", counter);
    println!("The result is {}", result);
}

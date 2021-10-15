fn main() {
    let a = [10, 20, 30, 40, 50];

    for elm in a.iter() {
        println!("{}", elm)
    }

    for i in (1..4).rev() {
        println!("{}!", i);
    }
    println!("LIFTOFF!!!");
}

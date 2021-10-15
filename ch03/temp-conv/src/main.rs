use std::io;
use std::io::Write;

fn main() {
    let mut fahrenheit = String::new();

    let fahrenheit: i32 = loop {
        print!("Enter temperature in freedom units: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut fahrenheit).expect("lol wut");
        let fahrenheit: i32 = match fahrenheit.trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        break fahrenheit;
    };

    println!("You entered {} in shit units", fahrenheit);

    let celsius = (fahrenheit - 32) * 5 / 9;
    println!("That's {}c in real units, shit brains", celsius);
}

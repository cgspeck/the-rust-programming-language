#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![1, 2, 3];

    let third = &v[2];

    println!("The third value is {}", third);

    match v.get(3) {
        Some(_third) => println!("there's a value at index 3"),
        _ => println!("no value at index 3"),
    };

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    // using an enum to store hetrogenous data within a Vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for r in &row {
        println!("{:#?}", r);
    }
}

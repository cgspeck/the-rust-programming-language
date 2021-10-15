fn main() {
    // types within tuple can be heterogeneous
    // type annotations optional
    let tup = (500, 6.4, 1);
    println!("{:#?}", tup);

    // destructure Tuple through pattern matching
    let (_x, y, _z) = tup;
    println!("the value of y is: {}", y);

    // indexed access
    println!("the value of the first element is: {}", tup.0);

    // types within array must be homogenous
    // arrays are of fixed length
    let _a = [1, 2, 3, 4, 5];

    // can also be declared with type and length
    let _b: [i32; 5] = [1, 2, 3, 4, 5];

    // initialize array of length 5, initial value 0
    let c = [0; 5];

    // index access
    println!("the value of the first element of array c is: {}", c[0]);
}

/*
Shadowing is different from marking a variable as mut, because we’ll
get a compile-time error if we accidentally try to reassign to this
variable without using the let keyword. By using let, we can perform
a few transformations on a value but have the variable be immutable
after those transformations have been completed.

The other difference between mut and shadowing is that because we’re
effectively creating a new variable when we use the let keyword again,
we can change the type of the value but reuse the same name. For
example, say our program asks a user to show how many spaces they want
between some text by inputting space characters, but we really want to
store that input as a number

Shadowing thus spares us from having to come up with different names,
such as spaces_str and spaces_num; instead, we can reuse the simpler
spaces name.

*/
fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);
}

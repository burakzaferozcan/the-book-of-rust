fn main() {
    println!("Hello, world!");
    another_function1();
    another_function2(5);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let z = plus_one(x);
    println!("The value of z is: {z}");
}
fn another_function1() {
    println!("Another function.");
}
//? Parametreler
fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}
//? Birden fazla parametre tanımlarken, parametreleri şu şekilde virgüllerle ayırın
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
//? Functions with Return Values
fn five() -> i32 {
    //todo return yazmamıza gerek yok
    5
}
fn plus_one(x:i32)->i32{
    return x+1
}
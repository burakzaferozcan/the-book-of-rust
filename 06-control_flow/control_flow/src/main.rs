fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
}

//todo if number {
//todo     println!("number was three");
//todo }
//? Hata, Rust'un a beklediğini boolancak bir tamsayı aldığını gösteriyor.
//? Ruby ve JavaScript gibi dillerden farklı olarak Rust,
//? Boolean olmayan türleri otomatik olarak Boolean'a dönüştürmeye çalışmaz.
//? Açık olmalı ve if koşulu olarak her zaman bir Boole değeri sağlamalısınız. 
//? Örneğin if kod bloğunun yalnızca bir sayı eşit olmadığında çalışmasını istiyorsak ,
//?  ifadeyi aşağıdaki şekilde değiştirebiliriz
if number != 0 {
    println!("number was something other than zero");
}

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}
let condition = true;
let number2 = if condition { 5 } else { 6 };

println!("The value of number is: {number2}");

let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};

println!("The result is {result}");

let mut count = 0;
'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
            break;
        }
        if count == 2 {
            break 'counting_up;
        }
        remaining -= 1;
    }

    count += 1;
}
println!("End count = {count}");

//? Koşullu Döngüler
let mut number3 = 3;
while number3 != 0 {
    println!("{number3}!");

    number3 -= 1;
}
println!("LIFT OFF!!!");

//? Bir Dizide Döngü Yapmak
let a = [10, 20, 30, 40, 50];
let mut index = 0;
while index < 5 {
    println!("the value is: {}", a[index]);
    index += 1;
}
let b = [10, 20, 30, 40, 50];
for element in b {
    println!("the value is: {element}");
}
for number in (1..4).rev() {
    println!("{number}!");
}
println!("LIFTOFF!!!");
}

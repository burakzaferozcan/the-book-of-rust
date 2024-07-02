fn main() {
    //? Mutability-Değişebilirlik
    //! mut kullanılmazsa yeni değer atanmaz
    //! let x=5
    let mut x = 8;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours have {THREE_HOURS_IN_SECONDS} seconds.");
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of x is: {y}");

    //? Hata, bir değişkenin türünü değiştirmemize izin verilmediğini söylüyor:
    //todo let mut spaces = "   ";
    //todo spaces = spaces.len();
    let spaces = " x  ";
    println!("spaces : {spaces}" );
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");

}
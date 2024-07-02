fn main() {
    //! DATA TYPES

    //? The Number Types
    let signed_byte1: i8 = -128;
    let signed_byte2: i8 = 127;
    let unsigned_byte1: u8 = 0;
    let unsigned_byte2: u8 = 255;
    let signed_short1: i16 = -32768;
    let signed_short2: i16 = 32767;
    let unsigned_short1: u16 = 0;
    let unsigned_short2: u16 = 65535;
    let signed_int1: i32 = -2147483648;
    let signed_int2: i32 = 2147483647;
    let unsigned_int1: u32 = 0;
    let unsigned_int2: u32 = 4294967295;
    let signed_long1: i64 = -9223372036854775808;
    let signed_long2: i64 = 9223372036854775807;
    let unsigned_long1: u64 = 0;
    let unsigned_long2: u64 = 18446744073709551615;
    let signed_double_long1: i128 = -170141183460469231731687303715884105728;
    let signed_double_long2: i128 = 170141183460469231731687303715884105727;
    let unsigned_double_long1: u128 = 0;
    let unsigned_double_long2: u128 = 340282366920938463463374607431768211455;
    let float32_min: f32 = std::f32::MIN;
    let float32_max: f32 = std::f32::MAX;
    let float64_min: f64 = std::f64::MIN;
    let float64_max: f64 = std::f64::MAX;
    let signed_size_min: isize = std::isize::MIN;
    let signed_size_max: isize = std::isize::MAX;
    let unsigned_size_min: usize = std::usize::MIN;
    let unsigned_size_max: usize = std::usize::MAX;

    println!("
    signed_byte1: {signed_byte1}
    signed_byte2: {signed_byte2}
    unsigned_byte1: {unsigned_byte1}
    unsigned_byte2: {unsigned_byte2}
    signed_short1: {signed_short1}
    signed_short2: {signed_short2}
    unsigned_short1: {unsigned_short1}
    unsigned_short2: {unsigned_short2}
    signed_int1: {signed_int1}
    signed_int2: {signed_int2}
    unsigned_int1: {unsigned_int1}
    unsigned_int2: {unsigned_int2}
    signed_long1: {signed_long1}
    signed_long2: {signed_long2}
    unsigned_long1: {unsigned_long1}
    unsigned_long2: {unsigned_long2}
    signed_double_long1: {signed_double_long1}
    signed_double_long2: {signed_double_long2}
    unsigned_double_long1: {unsigned_double_long1}
    unsigned_double_long2: {unsigned_double_long2}
    float32_min: {float32_min}
    float32_max: {float32_max}
    float64_min: {float64_min}
    float64_max: {float64_max}
    signed_size_min: {signed_size_min}
    signed_size_max: {signed_size_max}
    unsigned_size_min: {unsigned_size_min}
    unsigned_size_max: {unsigned_size_max}
    "
    );

    //? Numeric Operations
    let sum=5+10;
    println!("Sum: {sum}");
    let difference=10-5;
    println!("Difference: {difference}");
    let multiplication=5*10;
    println!("Multiplication: {multiplication}");
    let quotient=10/5;
    println!("Quotient division: {quotient}");
    let truncated = -5 / 3;
    println!("Truncated division: {truncated}");
    let remainder = 43 % 5;
    println!("Remainder: {remainder}");


    //? The Boolean Type
    let t=true;
    let f:bool=false;
    println!("t : {t}");
    println!("f : {f}");
    if t {println!("asd")}
    if f {println!("qwe")}


    //? The Character Type
    let c="z";
    println!("c : {c}");
    let z: char = 'ℤ'; 
    println!("z : {z}");
    let heart_eyed_cat = '😻';
    println!("Heart Eyed Cat: {heart_eyed_cat}");

    //? Compound Types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup : {:?}", tup);
    let (x, y, z) = tup;
    println!("The value of x:{x}, y:{y}, z:{z}");

    //? The Array Type
    let a = [1, 2, 3, 4, 5];
}

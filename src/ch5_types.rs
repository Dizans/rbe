// 类型转换 (casting)

// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

#[allow(dead_code)]
pub fn casting(){

    
    // 转为 无符号数
    // when casting any value to an unsigned type, T,
    // std::T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 如 1000 as u8  =>   1000 - 256 - 256 - 256 = 232 => 1000 % 256
    // -1 as u8 => -1 + 256 = 255
    let a: i32 = -1;
    println!("{}", a as u32);
    println!("{}", (-1i32) as u32);



    // 转为 有符号数    
    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.
    // 转换过程还是和转无符号数一样，所以bit级别的是相同的，只不过是现在要
    // 按照有符号数来理解

    println!("{}", 1000 as i8);
    println!("{}", (-10.99_f32) as i8);


    // 浮点数向整数转换
    /*         
        没有溢出, 则直接丢掉小数位
        溢出 则变成改类型符号整数的最大值或者最小值(由向上还是向下溢出决定)
    */
}

#[allow(dead_code)]
pub fn literals(){
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literal, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

#[allow(dead_code)]
fn aliasing(){
    type Inch = u64;

    // Types must have CamelCase names, or the compiler will raise a warning. 

    #[allow(non_camel_case_types)]
    type u64_t = u64;
}
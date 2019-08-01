// From and Into
use std::convert::From;

#[derive(Debug)]
struct Number{
    value: i32,
}

// The Into trait is simply the reciprocal of the From trait. 
// That is, if you have implemented the From trait for your type 
// you get the Into implementation for free.

impl From<i32> for Number{
    fn from(item: i32) -> Self{
        Number {value : item}
    }
}

pub fn from(){
    let my_str = "hello";
    let _my_string = String::from(my_str);
    let _my_string: String = my_str.into();

    let int = 6;

    let num = Number::from(int);
    println!("{:?}", num);
    let num: Number = int.into();
    println!("{:?}", num);
}

#[allow(dead_code)]
pub fn to_string(){
    // 想要获得 to_string 函数 
    // 你可以实现 ToString trait
    // 也可以通过实现 Display trait 自动获得

    use std::fmt;
    struct Circle{
        radius: i32,
    }
    impl fmt::Display for Circle{
        fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    let circle = Circle{radius: 6};
    println!("{}", circle.to_string());
}

#[allow(dead_code)]
pub fn parse_string(){
    // This will convert the string into the type specified 
    // so long as the FromStr trait is implemented for that type
    let _parsed: i32 = "5".parse().unwrap();
    let _turbo_parsed = "10".parse::<i32>().unwrap();
}
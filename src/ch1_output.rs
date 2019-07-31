use std::fmt;

/*
1. 输出有
    format!
    print!  println!
    eprint! eprintln!

2. Debug and Display
    fmt::Debug：使用 {:?} 标记。格式化文本以供调试使用。
    fmt::Display：使用 {} 标记。以更优雅和友好的风格来格式化文本。

    #[derive(Debug)]  可以得到默认实现

    Display 和 Debug 是要分开实现的，实现 Display 不会自动带来 Debug

3. 对于 {:b} 这种 要特殊实现，具体参看 std::fmt
    https://doc.rust-lang.org/std/fmt/#formatting-traits
 */

#[allow(dead_code)]
fn main() {
    // 有 直接替换 位置参数 命名参数 三种方式输出
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");

    practice1();
}

#[allow(dead_code)]
fn impl_display_for_struct(){
    // impl a Debug trait
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    impl fmt::Display for Person<'_>{
        fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
            write!(f, "my name is {}, I'm {} years old", self.name,self.age)
        }
    }

    let name = String::from("qingyu");
    let qingyu = Person{
        name: &name,
        age: 24
    };
    println!("{}", qingyu);
}

// https://doc.rust-lang.org/std/fmt/
#[allow(dead_code)]
pub fn practice1(){
    // 通过控制显示的小数位数来打印：Pi is roughly 3.142 
    let pi = 3.141592;
    println!("{:.*}",3,pi);
}


#[allow(dead_code)]
pub fn practice2(){
    struct List(Vec<i32>);

    impl fmt::Display for List{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            let vec = &self.0;

            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate(){
                if count!=0 {write!(f, ", ")?;}
                write!(f, "{}: {}", count, v)?;
            }

            write!(f, "]")
        }
    }

    let v = List(vec![1,2,3]);
    println!("{}",v );
}

#[allow(dead_code)]
pub fn practice3(){
    // 为 Color 结构体实现 fmt::Display
    struct Color{
        red: u8,
        green: u8,
        blue: u8,
    }

    impl fmt::UpperHex for Color{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f, "0x")?;
            write!(f, "{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
        }
    }
    
    impl fmt::Display for Color{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f, "RGB ({}, {}, {})", self.red, self.green, self.blue)?;
            write!(f, " {:X}", self)
        }        
    }

    let c = Color{
        red: 0,
        green: 3,
        blue: 254,
    };
    println!("{}", c);
}
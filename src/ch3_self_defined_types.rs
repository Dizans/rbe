/*
struct 的种类
    元组结构体（tuple struct），事实上就是具名元组而已。
    经典的 C 语言风格结构体（C struct）。
    单元结构体（unit struct），不带字段，在泛型中很有用。

初始化
    对于 Person 类的结构体,初始化很sb
        let name = "Peter";
        let age = 27;
        let peter = Person { name, age };
        如果变量名和字段名相同则不必写字段名,否则
            let peter = Person{name: 'qingyu', age: 21}

    更新式初始化
        let point: Point = Point { x: 0.3, y: 0.4 };
        let new_point = Point { x: 0.1, ..point };

    元组结构体
        let pair = Pair(1, 0.1);

解构结构体
    let Point { x: my_x, y: my_y } = point;
    这样就有了 my_x 和 my_y 两个变量


enum
    enum WebEvent {
        // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
        PageLoad,
        PageUnload,
        // 或者一个元组结构体，
        KeyPress(char),
        Paste(String),
        // 或者一个普通的结构体。
        Click { x: i64, y: i64 }
    }

    // 拥有显式辨别值（explicit discriminator）的 enum
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }




    
    
 */

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}


// 单元结构体

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段（field）的结构体
struct Point {
    x: f32,
    y: f32,
}


use List::*;

pub enum List{
    Cons(u32, Box<List>),
    Nil,
}



impl List{
    pub fn new() -> List {
        Nil
    }

    pub fn prepend(self, elem: u32) -> List{
        Cons(elem, Box::new(self))
    }

    pub fn len(&self) -> u32{
        match self{
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    pub fn stringify(&self) -> String{
        match *self{
            Cons(head,ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }

}
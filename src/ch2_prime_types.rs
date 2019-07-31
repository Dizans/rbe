use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// 切片是一个双字 对象（two-word object），第一个字是一个指向数据的指针，第二个字是切片的长度
// 数组的类型标记为 [T; size]
// slice 的类型标记为 &[T]

#[allow(dead_code)]
fn main(){
    use std::mem;

    let xs:[i32;5] = [1,2,3,4,5];
    let _ys: [i32;500] = [0; 500];

    println!("array occupies {} bytes", mem::size_of_val(&xs)); //byte 数
}

#[allow(dead_code)]
pub fn practice1(){
    impl fmt::Display for Matrix{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "({}, {})", self.0, self.1)?;
            writeln!(f, "({}, {})", self.2, self.3)
        }
    }
    let m = Matrix(1.2,2.1,3.1,4.1);
    println!("{}",m);
}


pub fn practice2(){
    // write tanspose function
    fn transpose(m: &Matrix) -> Matrix{
        Matrix(m.0,m.2,m.1,m.3)
    }
    let m = Matrix(1.2,2.1,3.1,4.1);
    println!("{}",m);
    println!("{}", transpose(&m));
}
//! SmartPoint
//! 
//!     这是第二行


#![allow(unused)]
#![allow(dead_code)]
use std::ops::Deref;


#[derive(Debug)]
struct Student{
    name:String,
    phone:String,
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &T {        
        & self.0
    }
}

impl<T> MyBox<T>{
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
}


/// # this is title
/// ## secend title
/// &emsp;&emsp;this is all part
///> hello
///>> workd 
///>>> hello
///>> heihei
/// _____________
/// 1. jkjk
/// 2. jkjk
/// * jkjkjk
/// # jkjkjk
/// [百度](https:www.baidu.com)
/// ```
/// let x = 5;
/// println!("x is {}",x);
/// 
/// ```
///     let x = t;    
/// 
/// 


pub fn smart_point(){
    {
        // let mut  s = Student{name:"suninglv".to_string(),phone:"15345923407".to_string()};
        // let mut x = Box::new(s);
        // println!("before modify the s.name is {},s.phone is {}",&x.name,&x.phone);
        // x.name = "jkjkjk".to_string();
        // x.phone = "898989898989".to_string();
        // println!("before the s.name is {},s.phone is{}",&x.name,&x.phone);
        // print!("x is {:?}",x);
    }
    
    {
        // let mut x = 5;
        // let mut x1 = Box::new(x);
        // *x1 = 7;
        // println!("{}",x1);

        // let mut s = Student{name:String::from("jkjkjk"),phone:String::from("jkjkjk")};
        // let mut s1 = Box::new(s);
        // s1.name = "jkjkjk".to_string();
    }
    
    {
        // let x = "rust".to_string();
        // let t = "rust".to_string();
        // let y = MyBox::new(x);
        // hello(&y);
    }


}
fn hello(x:&str){
    println!("{}",x);
}
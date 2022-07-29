#![allow(unused)]
#![allow(dead_code)]

#[derive(Debug)]
struct Student{
    name:String,
    phone:String,
}

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
    let mut x = 5;
    let mut x1 = Box::new(x);
    *x1 = 7;
    println!("{}",x1);

    let mut s = Student{name:String::from("jkjkjk"),phone:String::from("jkjkjk")};
    let mut s1 = Box::new(s);
    s1.name = "jkjkjk".to_string();

}
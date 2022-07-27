#![allow(dead_code)]
#![allow(unused)]
//#![allow(non_camel_case_types)]

use std::clone;
use core::fmt::Debug;

#[derive(Debug)]
pub struct Point<T> {
    x: T,
    y: T,
}
#[derive(Debug)]
pub struct Point2<T, U> {
    x: T,
    y: U,
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug, Clone)]
struct Student {
    name: String,
    age: u32,
    sex: String,
    phone: String,
    address: String,
}

#[derive(Debug, Clone)]
struct Teacher {
    name: String,
    age: u32,
    sex: String,
    phone: String,
    address: String,
}

pub trait GetInformation {
    fn get_name(&self) -> &str;
    fn get_age(&self) -> u32;
}


impl GetInformation for Student {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}
 
impl GetInformation for Teacher {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}
 
fn print_trait<T>(item: T)
where T: GetInformation
{
    println!("the name is:{} is from print_trait", item.get_name());
    println!("the age is:{} is from print_trait", item.get_age());
}

fn produce_item_trait() -> impl GetInformation + Debug
{
    Student {
        name: "suninglv".to_string(),
        age: 18,
        sex: "男".to_string(),
        phone: "15345923407".to_string(),
        address: "同安区洪塘镇".to_string(),
    }
}

pub fn struct_t() {
    //结构体定义及打印
    // let s1 = Point{x:32,y:64};
    // print!("the Point s1 is: {:#?}",s1);

    // let s2 = Point{x:32.09,y:64.89};
    // print!("the Point s2 is: {:#?}",s2);

    // let s3 = Point2{x:32.09,y:'c'};
    // print!("the Point s2 is: {:#?}",s3);
    //结构体方法
    let s = Student {
        name: String::from("suninglv"),
        age: 23,
        sex: String::from("男"),
        phone: String::from("15345923407"),
        address: String::from("tonganqu"),
    };

    let t = Teacher {
        name: String::from("wangwu"),
        age: 3,
        sex: String::from("男"),
        phone: String::from("15345923407"),
        address: String::from("tonganqu"),
    };

    let t2 = Teacher {
        name: String::from("lilei"),
        age: 3,
        ..t.clone()
    };

    println!("s is :{},{}", s.get_name(), s.get_age().to_string());
    println!("t is :{},{}", t.get_name(), t.get_age().to_string());
    print_trait(s);
    print_trait(t);
    let st = produce_item_trait();
    println!("st' name is:{:#?}", st);
}

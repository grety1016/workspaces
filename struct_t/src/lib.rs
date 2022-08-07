#![allow(dead_code)]
#![allow(unused)]
//#![allow(non_camel_case_types)]

use std::clone;
use core::fmt::Debug;

///定义结构及枚举类型
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
 
 
///定义学生及老师类 
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

///定义trait GetInformation
pub trait GetInformation:Debug {
    fn get_name(&self) -> &str;
    fn get_age(&self) -> u32;
}

///为学生及老师的类实现GetInformation
impl GetInformation  for Student {
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

///定义实现特定Trait的方法
fn print_trait<T>(item: T)
where T: GetInformation
{
    println!("the name is:{} is from print_trait", item.get_name());
    println!("the age is:{} is from print_trait", item.get_age());
}

///定义返回一个实现特定Trait的类型方法
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

///定义Trait GetName GetAge
trait GetName{
    fn get_name(&self) -> &String;
}

trait GetAge{
    fn get_age(&self) -> u32;
}

///为学生 老师实现trait GetName GetAge
impl GetName for Student {
    fn get_name(&self) -> &String{
        &self.name
    }
}

impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age    
    }
}

impl GetName for Teacher {
    fn get_name(&self) -> &String{
        &self.name
    }
}

impl GetAge for Teacher {
    fn get_age(&self) -> u32 {
        self.age    
    }
}

///定义泛型人类
struct People<T,U>{
    teacher:T,
    student:U,
}
///实现对某类型实现了指定某些Trait的方法
impl<T,U> People<T,U> 
    where T:GetAge + GetName,U:GetAge + GetName 
{
    fn print_people_info(&self){
        println!("teacher name is {}",self.teacher.get_name());
        println!("teacher age is {}",self.teacher.get_age());
        println!("student name is {}",self.student.get_name());
        println!("student age is {}",self.student.get_age());
    }

}
 
trait SecTrait {
    fn print_age(&self);
}

impl <T> SecTrait for T 
    where T:GetAge
{
    fn print_age(&self) {
        println!("the object age is {}",self.get_age());
    }
}
 

pub fn struct_t() {
    //结构体定义及打印
    // let s1 = Point{x:32,y:64};
    // print!("the Point s1 is: {:#?}",s1);

    // let s2 = Point{x:32.09,y:64.89};
     //print!("the Point s2 is: {:#?}",s2);

    // let s3 = Point2{x:32.09,y:'c'};
    // print!("the Point s2 is: {:#?}",s3);
    //结构体方法
    // let s = Student {
    //     name: String::from("suninglv"),
    //     age: 23,
    //     sex: String::from("男"),
    //     phone: String::from("15345923407"),
    //     address: String::from("tonganqu"),
    // };

    // let t = Teacher {
    //     name: String::from("wangwu"),
    //     age: 3,
    //     sex: String::from("男"),
    //     phone: String::from("15345923407"),
    //     address: String::from("tonganqu"),
    // };

    // let t2 = Teacher {
    //     name: String::from("lilei"),
    //     age: 3,
    //     ..t.clone()
    // };
    // //使用Trait方法 
    // println!("s is :{},{}", s.get_name(), s.get_age().to_string());
    // println!("t is :{},{}", t.get_name(), t.get_age().to_string());
    // //利用trait_print打印信息
    // print_trait(s);
    // print_trait(t);
    // //返回实现Trait的类型
    let st = produce_item_trait();
     println!("st' name is:{:#?}", st);
    //有条件实现方法，该方法必须实现特定Trait
    // let t = Teacher{
    //     name:"xiaowang".to_string(),
    //     age:12,
    //     phone:"1232323233".to_string(),
    //     address:"jkjkjkjkjk".to_string(),
    //     sex:"女".to_string(),
    // };
    // let s = Student{
    //     name:"xiaoguang".to_string(),
    //     age:33,
    //     phone:"1232323233".to_string(),
    //     address:"jkjkjkjkjk".to_string(),
    //     sex:"女".to_string(),
    // };

    // let p = People{
    //     teacher:t,
    //     student:s,
    // };

    // p.print_people_info();

    //有条件实现Trait
    let s1 = Student{
        name:"xiaoguang".to_string(),
        age:33,
        phone:"1232323233".to_string(),
        address:"jkjkjkjkjk".to_string(),
        sex:"女".to_string(),
    };    
    s1.print_age();






}

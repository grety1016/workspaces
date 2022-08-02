//! SmartPoint
//! 
//!     这是第二行


#![allow(unused)]
#![allow(dead_code)]
use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use std::option;
use std::rc::{Rc,Weak};
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

#[derive(Debug)] 
struct Student{
    name:String,
    phone:String,
}

#[derive(Debug)]
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





#[derive(Debug)]
enum List{
    Cons(i32,Box<List>),
    Nil,
}

#[derive(Debug)]
struct Dog{
    name:String,
    count:i32,
}

impl Drop for Dog{
    fn drop(&mut self){
        println!("Dog leave");
    }
}

#[derive(Debug)]
enum List2{
    Cons(i32,Rc<List2>),
    Nil,
}
#[derive(Debug)]
enum List3{
    Cons(Rc<RefCell<i32>>,Rc<List3>),
    Nil,
}
#[derive(Debug)]
enum List4{
    Cons(i32,RefCell<Rc<List4>>),
    Nil,
}


use List4::{Cons,Nil};
impl List4{    
    fn tail(&self) -> Option<&RefCell<Rc<List4>>>{
        match self {
            Cons(_,item) => Some(item),
            Nil => None,
        }

    }
}


#[derive(Debug)]
struct Node{
    value:i32,
    parent:RefCell<Weak<Node>>,
    children:RefCell<Vec<Rc<Node>>>,
}


pub fn smart_point(){
    //智能指针使用
    {
        // let mut  s = Student{name:"suninglv".to_string(),phone:"15345923407".to_string()};
        // let mut x = Box::new(s);
        // println!("before modify the s.name is {},s.phone is {}",&x.name,&x.phone);
        // x.name = "jkjkjk".to_string();
        // x.phone = "898989898989".to_string();
        // println!("before the s.name is {},s.phone is{}",&x.name,&x.phone);
        // print!("x is {:?}",x);
    }
    //智能指针实现对类的引用
    {
        // let mut x = 5;
        // let mut x1 = Box::new(x);
        // *x1 = 7;
        // println!("{}",x1);

        // let mut s = Student{name:String::from("jkjkjk"),phone:String::from("jkjkjk")};
        // let mut s1 = Box::new(s);
        // s1.name = "jkjkjk".to_string();
    }
//智能指针实现dref多态    
    {
        // let x = "rust".to_string();
        // let y = MyBox::new(x);
        // hello(&y);
    }
//使用智能指针实现链表   
    {
        // use List::{Cons,Nil};
        // let list = Cons(1, Box::new(Cons(2,
        //                             Box::new(Cons(3,
        //                                 Box::new(Nil))))));

        // println!("the list is: {:?} ",list);    
        // let x = MyBox::new(Cons(1, Box::new(Nil))); 
        // println!("the x is : {:?}",x);      

        
    }    
//提前调用drop
    {
        // let dog1 = Dog{name:String::from("dahunang"),count:1,};
        // drop(dog1);
        // println!("dog1 leave the region.");
    }
//使用rc智能指针
    {
    // use List2::{Cons,Nil};

    // let a = Rc::new(Cons(5, Rc::new(Nil)));
    // let b = Cons(6,Rc::clone(&a));
    // let c = Cons(7,a.clone());

    // println!("a: {:?},b:{:?},c:{:?}",a,b,c);


    }
//使用Rc::Refcell
    {
        // use List3::{Cons,Nil};
        // use std::rc::Rc;
        // use std::cell::RefCell;

        // let value = Rc::new(RefCell::new(5));

        // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil))); 
        // *value.borrow_mut() += 5;
        // println!("{:?}",&a);

    }    
//循环列表(出现死循环)
{
    // use std::rc::Rc;
    // use std::cell::RefCell;
    // use List4::{Cons,Nil};

    // let a = Rc::new(Cons(5,RefCell::new(Rc::new(Nil))));
    // println!("1, a rc count = {}",Rc::strong_count(&a));
    // println!("1, a tail = {:?}",a.tail());

    // let b = Rc::new(Cons(10,RefCell::new(Rc::clone(&a))));
    // println!("2, a rc count = {}",Rc::strong_count(&a));
    // println!("2, b rc count = {}",Rc::strong_count(&b));
    // println!("1, b tail = {:?}",b.tail());

    // if let Some(link) = a.tail(){
    //     *link.borrow_mut() = b.clone();
    // }
    // println!("2, a rc count = {}",Rc::strong_count(&a));
    // println!("2, b rc count = {}",Rc::strong_count(&b));
    //println!("1, a tail = {:?}",a.tail()); ////此代码造成死循环
    
}

{
    // let leaf = Rc::new(Node{value:3,parent:RefCell::new(Weak::new()),children:RefCell::new(vec![]),});
    // println!("leaf's parent is :{:?}",leaf.parent.borrow().upgrade());
    // println!("leaf's children is :{:?}",leaf.children.borrow());
    // let branch = Rc::new(Node{value:5,parent:RefCell::new(Weak::new()),children:RefCell::new(vec![Rc::clone(&leaf)]),});

    // *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // println!("leaf's parent is :{:?}",leaf.parent.borrow().upgrade());
    // println!("leaf's children is :{:?}",leaf.children.borrow());

    // println!("branch's parent is :{:?}",branch.parent.borrow().upgrade());
    // println!("branch's children is :{:?}",branch.children.borrow());



}


}
fn hello(x:&str){
    println!("{}",x);
}
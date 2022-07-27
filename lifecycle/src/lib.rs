#![allow(unused)]
#![allow(dead_code)]

fn longest<'a>(x:&'a str,y:&'a str) -> &'a str{
    if x.len() > y.len(){
        &x
    }else {
        &y
    }
}


struct Student<'a>{
    name:&'a str,
    sex:&'a str,
}

pub fn lifecycle(){
//    let s1 = String::from("abcde");
//    let s2 = String::from("dfdf");

//    let r=longest(&s1,&s2);
//    println!("the longest is :{}",r);

let s = Student{name:"jkjkjk",sex:"girl"};

println!("the student'name is :{},sex:{}",&s.name,&s.sex);
 



}




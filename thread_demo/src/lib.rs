#![allow(unused)]
#![allow(dead_code)]
//#![feature(scoped_thre&ads)]
use std::{time::Duration,thread,sync::mpsc};


pub fn thread_demo(){
    //使用子线程工作
    {
        // let handle = thread::spawn(||{
        // for i in 1..9{
        //     println!("in thread i is :{}",i);
        //     thread::sleep(Duration::from_millis(1));
        // }
        // });

        // for i in (1..4){
        //     println!("in main i is :{}",i);
        //     thread::sleep(Duration::from_millis(1));
        // }

        // handle.join().unwrap();
    }

    //使用作用域线程
    {
        //let local_var = vec![1, 2, 3];

        // println!("borrowed from thread #1: {:?}", local_var);
        // println!("borrowed from thread #2: {:?}", local_var);
        // println!("borrowed from the main thread: {:?}", local_var);

        //thread::scope(|s| {
        //    s.spawn(|| println!("borrowed from thread #1: {:?}", local_var));
        //    s.spawn(|| println!("borrowed from thread #2: {:?}", local_var));
        //    println!("borrowed from the main thread: {:?}", local_var);
        //});
        // let c = 'j';
        // match c {
        //     'a'..='z' => println!("yes{}",c),
        //     _ =>println!("no{}",c),
        // }
        
    }

}




#![allow(unused)]
use futures::future;
use futures::{select,future::FutureExt,pin_mut};
use tokio::runtime::Runtime;
use tokio::time;
use std::io::Result;


    ////2022-08-07 part1
    async fn func1() ->Result<()> {
        tokio::time::sleep(time::Duration::from_secs(3)).await;
        println!("func1 finished!");   
        Ok(()) 
    }

    async fn func2() ->Result<()> {
        println!("func2 finished!");
        Ok(())
    }

    async fn async_main(){
        let f1 = func1();
        let f2 = func2();    
        if let Err(_) =  futures::try_join!(f1,f2){
            println!("Err!");
        };
         
    }
    // async fn func1() -> Result<()> {
    //     tokio::time::sleep(time::Duration::from_secs(2)).await;
    //     println!("func1 finished!");
    //     Ok(())
    // }

    // async fn func2() -> Result<()> {
    //     println!("func2 finished!");
    //     Ok(())
    // }

    // async fn async_main() {
    //     let f1 = func1().fuse();
    //     let f2 = func2().fuse();
    //     pin_mut!(f1,f2);
 
    //     select! {
    //         _ = f1 => println!("func1 finished++++++"),
    //         _ = f2 => println!("func2 finished++++++"),
    //     }
    // } 

pub  fn async_demo() {
    let  runtime = Runtime::new().unwrap();
    runtime.block_on(async_main());

    // let   runtime = Runtime::new().unwrap();
    // runtime.block_on(async_main());  
    
}
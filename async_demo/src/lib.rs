use futures;
use tokio::runtime::Runtime;


async fn func1(){
    tokio::time::delay_for(tokio::timer::Duration::from_secs(1)).await;

}



pub  async fn async_demo() {
    
}
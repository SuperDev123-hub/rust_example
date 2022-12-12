use futures::executor::block_on;
use std::{thread, time};
async fn hello_world() {
    println!("start");
    for item in (1..9) {
        thread::sleep(time::Duration::from_secs(1));
        println!("hello - item, {:?}", item);
    }
}

async fn hello_world1() {
    println!("start");
    for item in (1..9) {
        thread::sleep(time::Duration::from_secs(1));
        println!("----------item, {:?}", item);
    }
}

async fn async_main() {
    let future_fn = hello_world();
    let future_fn2 = hello_world1();
    futures::join!(future_fn, future_fn2);
}
fn main() {    
    block_on(async_main());
}
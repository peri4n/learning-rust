use std::{thread, time};

use tokio::time::{sleep, Duration};

fn blocking_call() -> String {
    thread::sleep(time::Duration::from_secs(5));

    "Finally done".to_string()
}

async fn blocking_call2() {
    thread::sleep(time::Duration::from_secs(5));
}

async fn async_call(id: i32) {
    sleep(Duration::from_secs(1)).await;
    println!("Async Call: ID {}", id);
}

#[tokio::main]
async fn main() {
    println!("Starting of the program");
    println!("Demonstrate blocking tasks in a blocking-thread");

    let blocking_code_handle = tokio::task::spawn_blocking(blocking_call);

    let mut async_handles = Vec::new();
    for id in 0..10 {
        async_handles.push(tokio::spawn(async_call(id)));
    }

    println!("Executing threads");
    for handle in async_handles {
        handle.await.unwrap();
    }

    let result = blocking_code_handle.await.unwrap();
    println!("Blocking Call: {}", result);

    println!("Demonstrate blocking tasks in a non-blocking-thread");

    let mut async_handles = Vec::new();
    for id in 0..10 {
        async_handles.push(tokio::spawn(async_call(id)));
    }
    async_handles.push(tokio::spawn(blocking_call2()));

    println!("Executing threads");
    for handle in async_handles {
        handle.await.unwrap();
    }

    println!("Program finished");
}

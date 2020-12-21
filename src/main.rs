mod executor;
mod timer_future;
async fn hello()->String{
    String::from("hello word async")
}
fn main() {
    let (executor, spawner) = executor::new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer.
    spawner.spawn( async move {
        println!("howdy!");
        println!("{}",hello().await);
        // Wait for our timer future to complete after two seconds.
        timer_future::TimerFuture::new(std::time::Duration::new(2, 0)).await;
        println!("done!");
        
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();
}

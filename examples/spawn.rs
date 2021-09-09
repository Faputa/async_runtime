use async_runtime::spawn;

fn main() {
    futures::executor::block_on(async {
        // Spawn a future.
        let handle = spawn(async {
            println!("Running task...");
            1 + 2
        });

        // Await its output.
        assert_eq!(handle.await, 3);
    });
}

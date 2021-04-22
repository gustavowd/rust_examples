use std::thread;
use std::time::Duration;

fn main() {
    let x = std::sync::Arc::new(std::sync::Mutex::new(0_u32));
    let x_clone = x.clone();
    let handle = std::thread::spawn(move || {
        for i in 1..10 {
            let mut reference = x_clone.lock().unwrap();
            *reference = *reference + 2;
            println!("iter {}: value {} from the spawned thread!", i,*reference);
            drop(reference);
            thread::sleep(Duration::from_millis(10));
        }
    });

    {
        for i in 1..=10 {
            let mut reference = x.lock().unwrap();
            *reference = *reference + 1;
            println!("iter {}: value {} from the main thread!", i, *reference);
            drop(reference);
            thread::sleep(Duration::from_millis(10));
        }
    }

    handle.join().unwrap();
 
}

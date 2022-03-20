use event_listener::Event;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::usize;

pub fn ibento_risunaaa() {
    let flag = Arc::new(AtomicBool::new(false));
    let event = Arc::new(Event::new());

    // You can use this crate to turn non-blocking data structures into async or blocking data structures. See a simple mutex implementation that exposes an async and a blocking interface for acquiring locks.
    // このクレートを使って、ノンブロッキングなデータ構造を非同期またはブロッキングなデータ構造にすることができます。ロックを取得するための非同期とブロックのインタフェースを公開するシンプルなmutexの実装をご覧ください．

    // Spawn a thread that will set the flag after 1 second.
    thread::spawn({
        let flag = flag.clone();
        let event = event.clone();
        move || {
            // Wait for a second.
            println!("Start to sleep.");
            thread::sleep(Duration::from_secs(3));
            println!("Finished to sleep.");

            // Set the flag.
            println!("Do storing flag.");
            flag.store(true, Ordering::SeqCst);
            println!("Done storing flag.");

            // Notify all listeners that the flag has been set.
            println!("Do notify.");
            event.notify(usize::MAX);
            println!("Done notify.");
        }
    });

    // Wait until the flag is set.
    loop {
        // Check the flag.
        println!("Before loading flag.");
        if flag.load(Ordering::SeqCst) {
            println!("break.");
            break;
        }

        // Start listening for events.
        println!("Wait for creating event listener.");
        let listener = event.listen();
        println!("Done for creating event listener.");

        // Check the flag again after creating the listener.
        println!("Before loading flag.[2]");
        if flag.load(Ordering::SeqCst) {
            println!("break.[2]");
            break;
        }

        // Wait for a notification and continue the loop.
        println!("Wait for listening.");
        listener.wait();
        println!("Done for listening.");
    }
}

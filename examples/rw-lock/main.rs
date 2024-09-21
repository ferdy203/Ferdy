use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Config {
    setting: String,
}

fn main() {
    // Shared configuration, protected by RwLock.
    let config = Arc::new(RwLock::new(Config {
        setting: "Initial Configuration".to_string(),
    }));

    // Spawn a thread that reads the configuration periodically.
    let config_for_reader = Arc::clone(&config);
    let reader_handle = thread::spawn(move || {
        for _ in 0..5 {
            {
                // Acquire a read lock to read the configuration.
                let config = config_for_reader.read().unwrap();
                println!("Reader thread: Current config: {:?}", *config);
            }
            thread::sleep(Duration::from_secs(1)); // Simulate some work.
        }
    });

    // Simulate a configuration update after 2 seconds.
    thread::sleep(Duration::from_secs(2));
    {
        // Acquire a write lock to update the configuration.
        let mut config = config.write().unwrap();
        config.setting = "Updated Configuration".to_string();
        println!("Writer thread: Updated the config to: {:?}", *config);
    }

    // Wait for the reader thread to finish.
    reader_handle.join().unwrap();
}

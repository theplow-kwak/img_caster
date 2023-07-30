use std::io;
use std::io::Write; // <--- bring flush() into scope
use std::thread;
use std::time::Duration;

fn main() {
    for i in 1..=10 {
        print!("Progress: {}%", i);
        // Flush the output to make sure it's visible immediately
        let _ = std::io::stdout().flush();

        // Simulate some work being done
        thread::sleep(Duration::from_secs(1));

        // Clear the line (overwrite it with spaces) to prepare for the next iteration
        print!("\r{}", " ".repeat(12));
    }
    println!("\nTask completed!");
}
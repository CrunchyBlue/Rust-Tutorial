use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");
    v.iter()
        .filter(|&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];
    
    let handle = thread::spawn(move || expensive_sum(my_vector));
    
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", letter);
        pause_ms(200);
    }
    
    let sum = handle.join().unwrap();
    println!("The child thread's expensive sum is {}", sum);
    
    let (tx, rx) = channel::unbounded();
    
    let tx2 = tx.clone();
    
    let handle_a = thread::spawn(move || {
        pause_ms(0);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });
    
    pause_ms(100);
    
    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(200);
        tx.send("Thread B: 2").unwrap();
    });
    
    for msg in rx {
        println!("Main thread: Received {}", msg);
    }
    
    handle_a.join().unwrap();
    handle_b.join().unwrap();

    // Challenge: Make two child threads and give them each a receiving end to a channel.  From the
    // main thread loop through several values and print each out and then send it to the channel.
    // On the child threads print out the values you receive. Close the sending side in the main
    // thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`).  Join
    // the child threads.

    let (tx, rx) = channel::unbounded();

    let rx2 = rx.clone();

    let handle_c = thread::spawn(move || {
        for msg in rx2 {
            println!("Thread C: Received {}", msg);
        }
    });

    pause_ms(100);

    let handle_d = thread::spawn(move || {
        for msg in rx {
            println!("Thread D: Received {}", msg);
        }
    });

    for msg in ["Hello", "World", "How", "Are", "You?"] {
        println!("Main thread: Sent {}", msg);
        let _ = tx.send(msg);
    }
    
    drop(tx);
    
    handle_c.join().unwrap();
    handle_d.join().unwrap();
    
    println!("Main thread: Exiting.")
}

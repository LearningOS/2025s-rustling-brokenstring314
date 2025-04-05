// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);
    let txx = tx.clone();
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            txx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);
    // 在 Rust 的 mpsc 通道中，当所有的 Sender 实例都被销毁时，通道会被自动关闭。
    // 接收端（rx）在通道关闭后会返回 None，表示没有更多的消息可以接收。
    // 在你的代码中，当使用 tx.clone() 时，主线程中仍然保留了一个 tx 实例。
    // 即使两个线程中的 tx 和 txx 都发送完消息并被销毁，主线程中的 tx 仍然存在，因此通道不会被关闭。
    // 这导致主线程中的 for received in rx 循环会一直等待更多的消息，程序卡住。
    // 而当你使用 send_tx(queue, tx) 时，tx 的所有权被移动到了 send_tx 函数中，并且在两个线程中被克隆为 tx 和 txx。
    // 当这两个线程发送完消息并结束时，tx 和 txx 都被销毁，通道被关闭。
    // 主线程中的 for received in rx 循环会正确地接收到所有消息，并在通道关闭后退出循环。
    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}

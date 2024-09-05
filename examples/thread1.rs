use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread};

const NUM_PRODUCERS: usize = 4;

#[derive(Debug)]
#[allow(dead_code)]
struct Message {
    id: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("conmuser: {:?}", msg);
        }
    });

    let _ = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error : {:?}", e));

    Ok(())
}

fn producer(i: usize, tx: mpsc::Sender<Message>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Message::new(i, value))?;
        thread::sleep(std::time::Duration::from_millis(100));
    }
}

impl Message {
    fn new(id: usize, value: usize) -> Self {
        Self { id, value }
    }
}

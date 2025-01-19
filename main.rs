use tokio::sync::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct Agent {
    id: usize,
    position: (i32, i32),
}

impl Agent {
    fn new(id: usize) -> Self {
        Agent {
            id,
            position: (0, 0),
        }
    }

    fn move_agent(&mut self) {
        // Simple movement logic: move agents in random directions
        self.position.0 += 1; // move along X
        self.position.1 += 1; // move along Y
    }

    fn log_position(&self) {
        println!("Agent {} is at position {:?}", self.id, self.position);
    }
}

#[tokio::main]
async fn main() {
    let mut agents = vec![
        Arc::new(Mutex::new(Agent::new(1))),
        Arc::new(Mutex::new(Agent::new(2))),
    ];

    loop {
        for agent in &agents {
            let mut agent = agent.lock().await;
            agent.move_agent();
            agent.log_position();
        }

        // Sleep for a bit before the next cycle
        thread::sleep(Duration::from_secs(1));
    }
}

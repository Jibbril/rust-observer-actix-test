mod message_payload;
mod participant;
mod random_generator;
mod scoreboard;

use actix::Actor;
use participant::Participant;
use random_generator::RandomGenerator;
use scoreboard::ScoreBoard;
use tokio::time::{sleep, Duration};

#[actix::main]
async fn main() {
    let scoreboard = ScoreBoard {
        id: 1,
        participants: vec![
            Participant::new("A"),
            Participant::new("B"),
            Participant::new("C"),
        ],
    };
    let addr = scoreboard.start();

    for i in 1..10 {
        let random_gen = RandomGenerator {
            id: i,
            observer: addr.clone(),
        };
        let _rand_addr = random_gen.start();
    }

    // Prevent the program from exiting immediately
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}

mod message_payload;
mod participant;
mod random_generator;
mod scoreboard;

use actix::Actor;
use participant::Participant;
use random_generator::RandomGenerator;
use scoreboard::ScoreBoard;

// #[actix::main]
// async fn main() {
//     let scoreboard = ScoreBoard {
//         id: 1,
//         participants: vec![
//             Participant::new("Alice"),
//             Participant::new("Bob"),
//             Participant::new("Charlie"),
//         ],
//     };
//     let addr = scoreboard.start();

//     let random_gen = RandomGenerator { id: 1, observer: addr.clone() };
//     let _rand_addr = random_gen.start();
// }
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
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

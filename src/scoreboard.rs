use actix::{Actor, Context, Handler};
use rand::random;

use crate::{message_payload::MessagePayload, participant::Participant};

#[derive(Debug, Clone)]
pub struct ScoreBoard {
    pub id: u64,
    pub participants: Vec<Participant>,
}

#[allow(dead_code)]
impl ScoreBoard {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            participants: vec![],
        }
    }

    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant);
    }
}

impl Actor for ScoreBoard {
    type Context = Context<Self>;
}

impl Handler<MessagePayload> for ScoreBoard {
    type Result = ();

    fn handle(&mut self, msg: MessagePayload, _ctx: &mut Self::Context) -> Self::Result {
        let index = random::<usize>() % self.participants.len();
        self.participants[index].score += msg.0;

        let mut scores_line = String::new();
        for participant in self.participants.iter() {
            scores_line.push_str(&format!("{: <3} ", participant.score));
        }
        println!("{}", scores_line);
    }
}

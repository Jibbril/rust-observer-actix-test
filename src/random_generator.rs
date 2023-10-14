use crate::{message_payload::MessagePayload, scoreboard::ScoreBoard};
use actix::clock::sleep;
use actix::{
    fut::ready, prelude::ContextFutureSpawner, Actor, ActorFutureExt, Addr, Context, WrapFuture,
};
use rand::random;
use std::time::Duration;

#[derive(Clone)]
pub struct RandomGenerator {
    pub id: u64,
    pub observer: Addr<ScoreBoard>,
}

impl RandomGenerator {
    fn generate(&self, ctx: &mut Context<Self>) {
        let sleep_time = random::<u64>() % 10 + 1;

        sleep(Duration::from_secs(sleep_time))
            .into_actor(self)
            .then(move |_, act, ctx| {
                act.observer.do_send(MessagePayload(sleep_time));
                act.generate(ctx);
                ready(())
            })
            .wait(ctx);
    }
}

impl Actor for RandomGenerator {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.generate(ctx);
    }
}

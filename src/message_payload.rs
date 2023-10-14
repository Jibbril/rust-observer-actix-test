use actix::Message;

pub struct MessagePayload(pub u64);

impl Message for MessagePayload {
    type Result = ();
}

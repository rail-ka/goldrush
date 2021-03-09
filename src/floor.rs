use actix::prelude::*;

pub struct Floor {
    pub floor: usize,
}

impl Actor for Floor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct FloorMessage(u32);

impl Handler<FloorMessage> for Floor {
    type Result = usize;

    fn handle(&mut self, msg: FloorMessage, ctx: &mut Context<Self>) -> Self::Result {
        1
    }
}

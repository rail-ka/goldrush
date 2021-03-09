use actix::prelude::*;
use awc::FrozenClientRequest;
use crate::models::Area;
use crate::requests::explore;

pub struct Explorer {
    builder_explore: FrozenClientRequest
}

impl Actor for Explorer {
    type Context = Context<Self>;
}

impl Handler<Area> for Explorer {
    type Result = usize;

    fn handle(&mut self, msg: Area, ctx: &mut Context<Self>) -> Self::Result {
        // let res = explore(&self.builder_explore, area).await;
        1
    }
}

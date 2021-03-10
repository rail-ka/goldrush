use actix::prelude::*;
use actix::AsyncContext;
use awc::FrozenClientRequest;
use crate::models::Area;
use crate::requests::explore;
use futures::task::SpawnExt;

// pub struct Explorer {
//     builder_explore: FrozenClientRequest
// }
//
// impl Actor for Explorer {
//     type Context = Context<Self>;
// }
//
// impl Handler<Area> for Explorer {
//     type Result = usize;
//
//     fn handle(&mut self, msg: Area, ctx: &mut Self::Context) -> Self::Result {
//         let f = async move {
//             let res = explore(&self.builder_explore, msg).await;
//         };
//         // let fut = actix::fut::wrap_future::<_, Self>(f);
//         // f.into_actor(self).spawn(ctx);
//         // ctx.spawn(f);
//         1
//     }
// }

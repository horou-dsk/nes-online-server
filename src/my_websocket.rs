use std::time::{Instant, Duration};
use actix::{Actor, AsyncContext, ActorContext, StreamHandler, Addr, WrapFuture, ActorFuture, ContextFutureSpawner, fut, Running, Handler};
use actix_web_actors::ws;
use actix_web::{HttpRequest, web, HttpResponse, Error};
use crate::online_server as server;
use log::{info};
use serde_json::Value;
use crate::online_server::{ClientMessage, RoomMessage, SingleMessage};
use crate::proto::{SendParcel};
use actix_web_actors::ws::WebsocketContext;
use actix_http::ws::Codec;
use crate::recv_proto::ControllerIns;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct MyWebSocket {
    /// unique session id
    id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    /// joined room
    room: String,
    /// peer name
    name: Option<String>,
    /// Chat server
    addr: Addr<server::OnLineServer>,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
        println!("阻塞了吗？");
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<server::Message> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        // println!("WEBSOCKET MESSAGE: {:?}", msg);
        // let elapsed = Instant::now();
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let v: Value = serde_json::from_str(text.as_str()).unwrap();
                match v["mid"].as_u64() {
                    Some(1) => {
                        self.addr.do_send(RoomMessage {
                            id: self.id,
                            room: self.room.clone(),
                        });
                    }
                    Some(4) => {
                        self.addr.do_send(ClientMessage {
                            id: 0,
                            room: self.room.clone(),
                            msg: serde_json::to_string(&SendParcel::GameStart(self.id)).unwrap(),
                        })
                        // self.addr.do_send(ClientMessage {
                        //     id: self.id,
                        //     room: self.room.clone(),
                        //     msg: serde_json::to_string(&SendParcel::GetGameStatus(self.id)).unwrap(),
                        // })
                    }
                    // 获取游戏进度
                    Some(5) => {
                        self.addr.do_send(SingleMessage {
                            id: v["id"].as_u64().unwrap() as usize,
                            msg: serde_json::to_string(&SendParcel::SendGameStatus(v["json_data"].to_string())).unwrap()
                        })
                    }
                    // 进度加载完毕
                    Some(6) => {
                        self.addr.do_send(ClientMessage {
                            id: self.id,
                            room: self.room.clone(),
                            msg: serde_json::to_string(&SendParcel::GameStart(self.id)).unwrap(),
                        })
                    }
                    Some(10) => {
                        let b = serde_json::from_value::<ControllerIns>(v).unwrap();
                        self.addr.do_send(ClientMessage {
                            id: self.id,
                            room: self.room.clone(),
                            msg: serde_json::to_string(&SendParcel::SendKeyboard(b.keys)).unwrap(),
                        })
                    }
                    _ => {
                        info!("无效消息");
                    }
                }
            }
            ws::Message::Binary(bytes) => {
                // let v = bytes.to_vec();
                // self.addr.do_send(ClientMessage {
                //     id: self.id,
                //     room: self.room.clone(),
                //     msg: serde_json::to_string(&SendParcel::SendKeyboard(v)).unwrap(),
                // })
            },
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
        // println!("{:?}", elapsed.elapsed())
    }
}

impl MyWebSocket {
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                act.addr.do_send(server::Disconnect { id: act.id });
                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

/// do websocket handshake and start `MyWebSocket` actor
pub async fn ws_index(r: HttpRequest, stream: web::Payload, srv: web::Data<Addr<server::OnLineServer>>) -> Result<HttpResponse, Error> {
    info!("新连接·····");
    let mut res = ws::handshake(&r)?;
    Ok(res.streaming(WebsocketContext::with_codec(MyWebSocket {
        id: 0,
        hb: Instant::now(),
        room: "Main".to_owned(),
        name: None,
        addr: srv.get_ref().clone(),
    }, stream, Codec::new().max_size(1024 * 1024 * 6))))
    // let res = ws::start(, &r, stream);
    // res
}

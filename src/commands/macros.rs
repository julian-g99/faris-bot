#[macro_export]
macro_rules! send_msg {
    ($x: expr, $msg: expr, $ctx: expr) => {
        let _ = $msg.channel_id.say(&$ctx.http, $x);
    }
}

use chrono::prelude::*;
use serenity::framework::standard::{macros::command, ArgError, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub fn time(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let time = match args.remaining() {
        0 => {
            // should return the UTC time for EST
            //let utc = Utc::now();

            //let (date, time) = (utc.date(), utc.time());
            Utc::now().time()
        }
        1 => {
            let hour = 3600;
            let timezone: i32 = args.single()?;
            let datetime = FixedOffset::east(timezone * hour);
            (Utc::now() + datetime).time()
        }
        _ => {
            send_msg!("Please specify either one or no argument", msg, ctx)?;
            return Ok(());
        }
    };
    //let time = utc.time();
    let time_str = time.format("%I:%M:%S %p").to_string();
    send_msg!(format!("The current time is {}", time_str), msg, ctx)?;
    Ok(())
}

#[command]
pub fn date(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let date = match args.remaining() {
        0 => {
            // should return the UTC time for EST
            //let utc = Utc::now();

            //let (date, time) = (utc.date(), utc.time());
            Utc::now().date()
        }
        1 => {
            let hour = 3600;
            let timezone: i32 = args.single()?;
            let datetime = FixedOffset::east(timezone * hour);
            (Utc::now() + datetime).date()
        }
        _ => {
            send_msg!("Please specify either one or no argument", msg, ctx)?;
            return Ok(());
        }
    };
    //let time = utc.time();
    let time_str = date.format("%Y-%m-%d").to_string();
    send_msg!(format!("The current date is {}", time_str), msg, ctx)?;
    Ok(())
}

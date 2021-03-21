include!("token.rs");
#[macro_use]
extern crate lazy_static;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use regex::Regex;

use rand::Rng;

struct Handler;

fn max(max_val:i64, val:i64) -> i64{
    return if val>max_val {val} else {max_val};
}

fn min(min_val:i64, val:i64) -> i64{
    return if val<min_val || min_val == 0{val} else {min_val};
}


fn str_to_roll_type (roll_type_str : &str) -> Option<(i64, fn(i64,i64)->i64)>{
    if roll_type_str == "A"{
        Some((3, max))
    }else if roll_type_str == "B"{
        Some((2, max))
    }else if roll_type_str == "C"{
        Some((1, max))
    }else if roll_type_str == "D"{
        Some((2, min))
    }else if roll_type_str == "E"{
        Some((3, min))
    }else{
        None
    }
}

fn roll(num : usize, num_per_roll: i64, operator : fn(i64,i64)->i64) -> Vec<i64> {
    let mut roll_result : Vec<i64> = Vec::new();
    roll_result.reserve(num);
    for _ in 0..num {
        let mut max = 0;
        for i in 0..num_per_roll{
            let val = rand::thread_rng().gen_range(1..21);
            max = operator(max,val);
        }
        roll_result.push(max);
    }
    roll_result
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        lazy_static!{
            static ref RE : Regex = Regex::new(r"(![jJ]amie)(\s*?)(\d{1,3})(\s*?)([A-E]{1})$")
                                           .unwrap_or_else(|_| panic!("Invalid regex!"));
        }
        let regex_groups = RE.captures(&msg.content);
        if regex_groups.is_some(){
            let regex_groups = regex_groups.unwrap();
            let num_of_rolls : usize = regex_groups.get(3)
                                                   .unwrap()
                                                   .as_str()
                                                   .parse()
                                                   .unwrap();
            let roll_type_str = regex_groups.get(5).unwrap().as_str();
            let roll_type = str_to_roll_type(roll_type_str).unwrap();
            let roll_result = roll(num_of_rolls, roll_type.0, roll_type.1);
            let reply = format!("{:?}", roll_result);
            if let Err(why) = msg.channel_id.say(&ctx.http, reply).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let mut client = Client::builder(&TOKEN)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

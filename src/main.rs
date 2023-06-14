use std::collections::HashMap;
use std::time::Duration;
use std::thread::sleep;

// use serde_json;
use reqwest;
use serde_json::Value;

static CHAT_ID:i64=1002065529;
static API_KEY:&str="2096695690:AAEsMWmKlFsfZ_NEaR31PpdhNepsfUtjVRM";


fn send_message(message:String)->String{
    let mut data=HashMap::new();
    data.insert("chat_id", CHAT_ID.to_string());
    data.insert("text", message);
    data.insert("parse_mode", String::from("HTML"));

    let client = reqwest::blocking::Client::new();
    let res = client.post(format!("https://api.telegram.org/bot{}/sendMessage", API_KEY))
                            .json(&data)
                            .send()
                            .unwrap()
                            .text()
                            .expect("well, it didn't work.");

    return res;
}

struct Message{
    text:String,
    update_id:i64
}

fn wait_new_message(last_id:i64)->Message{
    let url = format!("https://api.telegram.org/bot{}/getUpdates?chat_id={}&offset={}", API_KEY, CHAT_ID, last_id);
    let client = reqwest::blocking::Client::new();

    let mut message_contents:String=String::new();
    let mut message_id:i64=last_id;

    while true {
        let response: Value = client.get(&url)
                                    .send()
                                    .unwrap()
                                    .json::<serde_json::Value>()
                                    .expect("getting messages failed.");
                                
        let updates=response["result"].as_array().unwrap();
        let last_update=&updates[updates.len()-1];

        println!("{} | {} - {}", updates.len(), updates[updates.len()-1]["update_id"].as_i64().unwrap(), updates[0]["update_id"].as_i64().unwrap());

        let tmp_id=last_update["update_id"].as_i64().unwrap();

        println!("{:?}\n{} {}",last_update, message_id, tmp_id);

        if tmp_id<=message_id || message_id==0 {
            message_id=tmp_id;
        } else {
            message_contents=String::from(last_update["message"]["text"].as_str().unwrap());
            
            break;
        }

        sleep(Duration::from_secs(5));        
    }

    Message{text: message_contents, update_id: message_id}
}

fn main() {
    // let res=send_message(String::from("message"), tg_api_key, tg_chat_id);
    let mut last_id=0;
    while true {
        let _ = send_message(String::from("loop"));
        let res=wait_new_message(last_id);

        println!("{}",res.text);

        last_id=res.update_id;
    }
}

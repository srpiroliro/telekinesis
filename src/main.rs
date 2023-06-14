use std::collections::HashMap;
use std::fmt::format;
use std::time::Duration;
use std::thread::sleep;
use std::process::Command;

// use serde_json;
use reqwest;
use serde_json::Value;

static CHAT_ID:i64=0;
static API_KEY:&str="***REMOVED***";
static MSG_LIMIT:i16=4_000;

struct Message{
    text:String,
    update_id:i64
}


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

fn wait_new_message(last_id:i64)->Message{
    let url = format!("https://api.telegram.org/bot{}/getUpdates?chat_id={}&offset={}", API_KEY, CHAT_ID, last_id);
    let client = reqwest::blocking::Client::new();

    let message_contents:String;
    let mut message_id:i64=last_id;

    loop {
        let response: Value = client.get(&url)
                                    .send()
                                    .unwrap()
                                    .json::<serde_json::Value>()
                                    .expect("getting messages failed.");
                                
        let updates=response["result"].as_array().unwrap();
        let last_update=&updates[updates.len()-1];
        let tmp_id=last_update["update_id"].as_i64().unwrap();

        if tmp_id>message_id && message_id!=last_id {
            message_contents=String::from(last_update["message"]["text"].as_str().unwrap());
            
            break;
        } else if tmp_id>message_id {
            message_id=tmp_id
        }

        sleep(Duration::from_secs(5));        
    }

    Message{text: message_contents, update_id: message_id}
}


fn n_splitter(target:String, n_chars:i16)->Vec<String>{
    let mut slice:String=String::new();
    let mut groups:Vec<String>=Vec::new();

    for word in target.split(" ") {
        if word.len()+1+slice.len() > n_chars as usize {
            groups.push(slice);
            slice=String::new();
        } else {
            slice.push_str(format!("{} ",word).as_str());
        }
    }

    if !slice.is_empty() {
        groups.push(slice);
    }

    return groups;
}


fn main() {
    let mut last_id=0;  

    loop {
        send_message(String::from(">>>"));
        let res=wait_new_message(last_id);
        let mut arguments: Vec<&str>=res.text.split_whitespace().collect::<Vec<_>>();
        let command = arguments.remove(0);

        println!("command: {} | args: {:?}",command, arguments);

        let execution = Command::new(command)
                                    .args(arguments)
                                    .output();

        match execution {
            Err(e) => {
                send_message(format!("error: \"{}\"", e));
            },
            Ok(execution) => {

                let stdout = String::from_utf8_lossy(&execution.stdout);
                let stderr = String::from_utf8_lossy(&execution.stderr);
                
                println!("out: {}", stdout);
                println!("error: {}", stderr);
                
                if !stdout.is_empty() {
                    let splitted_out = n_splitter(stdout.to_string(), MSG_LIMIT);
                    
                    for msg in splitted_out {
                        send_message(msg);
                    }
                }
                
                if !stderr.is_empty() {
                    let splitted_err = n_splitter(stderr.to_string(), MSG_LIMIT);
                    for msg in splitted_err {
                        send_message(msg);
                    }
                }
            }
        }

        last_id=res.update_id;
    }
}
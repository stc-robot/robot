use starcoin_rpc_client::RpcClient;

use std::{sync::Mutex};
use once_cell::sync::Lazy;


const URL: &str = "ws://main.seed.starcoin.org:9870";

static CLIENT: Lazy<Mutex<RpcClient>> = Lazy::new(|| {
    let client =  RpcClient::connect_websocket(URL);
    Mutex::new(client.unwrap())
});







pub fn create(){
    let client =  CLIENT.lock().unwrap().account_create("".to_string());
    println!("client {:?}",client);
}
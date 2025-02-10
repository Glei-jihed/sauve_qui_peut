mod network;
mod game;

use std::sync::{Arc, Mutex};
use std::thread;
use game::GameClient;

fn main() {
    let server_address = "127.0.0.1:8778";
    let mut client = GameClient::new(server_address);
    
    client.register_team("TeamRust");
    client.subscribe_player("Player1");

    let stream = Arc::new(Mutex::new(client.stream.try_clone().unwrap()));

    // Thread pour écouter les messages du serveur
    let stream_clone = Arc::clone(&stream);
    thread::spawn(move || {
        loop {
            let mut stream_guard = stream_clone.lock().unwrap();
            let response = network::read_message(&mut *stream_guard);
            println!("Server says: {}", response);
        }
    });

    // Thread principal pour gérer les actions du joueur
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let mut stream_guard = stream.lock().unwrap();
        network::send_message(&mut *stream_guard, &input.trim());
    }
}

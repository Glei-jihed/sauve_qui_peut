mod game;
mod network;

use crate::game::GameClient;

fn main() {
    let server_address = "127.0.0.1:8778";
    let mut client = GameClient::new(server_address);

    client.register_team("team_test");
}

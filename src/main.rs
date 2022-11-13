#![windows_subsystem = "windows"]
mod api;
use std::error::Error;
use api::api_types::LolLobbyLobbyDto;
use shaco;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use serde::Serialize;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn write_to_clipboard(out_string: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx = ClipboardContext::new()?;
    ctx.set_contents(out_string.to_owned())?;
    Ok(())
}

fn write_json_to_clipboard<T: Serialize>(json: &T) -> Result<(), Box<dyn Error>> {
    let out_string = serde_json::to_string(json)?;
    write_to_clipboard(&out_string)
}

#[tokio::main]
async fn main() {
    let client = shaco::rest::RESTClient::new().expect("Failed to create client.");

    // Get lobby info
    let endpoint = "/lol-lobby/v2/lobby";

    let lobby: LolLobbyLobbyDto = serde_json::from_value(
        client
        .get(endpoint.to_string())
        .await
        .expect("Failed to fetch from endpoint.")
    ).expect("Failed to deserialize lobby json");

    write_json_to_clipboard(&lobby).expect("Failed to write to clipboard");

    let num_players = lobby.members.len();

    let mut temp: Vec<_> = (0..num_players).collect();

    temp.shuffle(&mut thread_rng());

    let mut team_blue = ".\nTeam 1:\n".to_owned();
    let mut team_red = "Team 2:\n".to_owned();

    for i in 0..num_players {
        let summoner_name = lobby.members[i].summonerName.clone();
        let with_new_line = format!("{summoner_name}\n");
        if temp[i] % 2 == 0 {
            team_blue.push_str(&with_new_line);
        } else {
            team_red.push_str(&with_new_line);
        }
    }

    team_blue.push_str("----------------------\n");
    team_blue.push_str(&team_red);

    write_to_clipboard(&team_blue).expect("Failed to write to clipboard");
}

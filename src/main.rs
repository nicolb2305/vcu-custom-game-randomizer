#![windows_subsystem = "windows"]
mod api;
use api::api_types::*;
use shaco;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn find_custom_game_chat(conversations: Vec<LolChatConversationResource>) -> Option<LolChatConversationResource> {
    for conv in conversations {
        if conv.type_ == "customGame" {
            return Some(conv);
        }
    }
    None
}

#[tokio::main]
async fn main() {
    // Create client
    let client = shaco::rest::RESTClient::new().unwrap();

    // Get lobby info
    let lobby: LolLobbyLobbyDto = serde_json::from_value(
        client
        .get("/lol-lobby/v2/lobby".to_string())
        .await
        .expect("Failed to fetch from endpoint.")
    ).expect("Failed to deserialize lobby json");

    // Randomize players
    let num_players = lobby.members.len();
    let mut player_iterator: Vec<_> = (0..num_players).collect();
    player_iterator.shuffle(&mut thread_rng());

    let mut team_blue = ".\nTeam 1:\n".to_owned();
    let mut team_red = "Team 2:\n".to_owned();

    for i in 0..num_players {
        let summoner_name = &lobby.members[i].summonerName;
        let with_new_line = format!("{summoner_name}\n");
        if player_iterator[i] % 2 == 0 {
            team_blue.push_str(&with_new_line);
        } else {
            team_red.push_str(&with_new_line);
        }
    }

    team_blue.push_str("----------------------\n");
    team_blue.push_str(&team_red);

    // Find custom game chat
    let conversations: Vec<LolChatConversationResource> = serde_json::from_value(
        client
            .get("/lol-chat/v1/conversations".to_string())
            .await
            .unwrap()
    ).unwrap();

    let lobby_conv = find_custom_game_chat(conversations).unwrap();

    // Post teams to custom game chat
    let lobby_id = lobby_conv.id;
    let endpoint = format!("/lol-chat/v1/conversations/{lobby_id}/messages");

    let post_body = LolChatConversationMessageResource {
        body: team_blue,
        type_: "groupchat".to_string(),
        fromId: None,
        fromPid: None,
        fromSummonerId: None,
        id: None,
        isHistorical: None,
        timestamp: None,
    };

    let post_body_string = serde_json::to_value(&post_body).unwrap();
    client.post(endpoint, post_body_string).await.unwrap();
}

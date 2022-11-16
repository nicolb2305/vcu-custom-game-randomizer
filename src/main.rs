#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows")
]
use shaco::rest::RESTClient;
use shaco::api::api_endpoints::{get_lol_lobby_v2_lobby, get_lol_chat_v1_conversations, post_lol_chat_v1_conversations_by_id_messages};
use shaco::api::api_types::{LolChatConversationMessageResource, LolChatConversationResource};
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
    let client = RESTClient::new().unwrap();

    // Get lobby info
    let lobby = get_lol_lobby_v2_lobby(&client).await.unwrap(); 

    // Randomize players
    let num_players = lobby.members.len();
    let mut player_iterator: Vec<_> = (0..num_players).collect();
    player_iterator.shuffle(&mut thread_rng());

    let mut team_blue = ".\nTeam 1:\n".to_owned();
    let mut team_red = "Team 2:\n".to_owned();

    for i in 0..num_players {
        let summoner_name = &lobby.members[i].summoner_name;
        let with_new_line = format!("{summoner_name}\n");
        match player_iterator[i] % 2 {
            0 => team_blue.push_str(&with_new_line),
            1 => team_red.push_str(&with_new_line),
            _ => ()
        }
    }

    team_blue.push_str("----------------------\n");
    team_blue.push_str(&team_red);

    // Find custom game chat
    let conversations = get_lol_chat_v1_conversations(&client).await.unwrap();

    let lobby_conv = find_custom_game_chat(conversations).unwrap();

    // Post teams to custom game chat
    let lobby_id = lobby_conv.id;

    let post_body = LolChatConversationMessageResource {
        body: team_blue,
        type_: "groupchat".to_string(),
        from_id: "".to_string(),
        id: "".to_string(),
        from_summoner_id: 0,
        from_obfuscated_summoner_id: 0,
        from_pid: "".to_string(),
        timestamp: "".to_string(),
        is_historical: false
    };

    post_lol_chat_v1_conversations_by_id_messages(&client, lobby_id, post_body).await.unwrap();
}

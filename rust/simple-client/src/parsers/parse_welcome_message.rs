use quick_xml::events::BytesStart;

use crate::structs::game_data::GameData;

pub fn parse_welcome_message(e: &BytesStart, game_data: &mut GameData) {
    let team = String::from_utf8(e.try_get_attribute("color").unwrap().unwrap().value.to_vec()).unwrap();
    println!("Received welcome message");
    println!("Our team: {}", &team);
    game_data.set_team(team.as_str());
}
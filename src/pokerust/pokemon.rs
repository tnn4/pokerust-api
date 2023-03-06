use serde::{Serialize, Deserialize};

use serde_json::Result;

use url::{Url, ParseError};


#[derive(Serialize, Deserialize, Debug)]
struct Pokemon {
    name: String,
    national_pokedex_number: i16,
    poketype: Vec<[PokeType; 2]>,
    stats: PokeStats,
    image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum PokeType {
    Fire,
    Grass,
    Water,
    Flying,
}

#[derive(Serialize, Deserialize, Debug)]
struct PokeStats {
    hp: i32,
    atk: i32,
    def: i32,
    spatk: i32,
    spdef: i32,
    spd: i32,
}


pub fn create_json_by_serializing_data_structures() -> Result<()> {
    // some data structure
    let charizard = Pokemon {
        name: "charizard".to_string(),
        national_pokedex_number: 6,
        poketype: vec!([PokeType::Fire, PokeType::Flying]),
        stats: PokeStats {
            hp: 78,
            atk: 84,
            def: 78,
            spatk: 109,
            spdef: 85,
            spd: 100,
        },
        image_url: "https://bulbapedia.bulbagarden.net/wiki/File:0006Charizard.png".to_string(),
    };
    // Serialize to JSON string
    let json_from_pokemon = serde_json::to_string(&charizard)?;

    // Print, write to file or send to HTTP server

    println!("{}", json_from_pokemon);

    Ok(())
}
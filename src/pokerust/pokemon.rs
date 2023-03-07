use std::fs::File;
use std::io::prelude::*;

use serde::{Serialize, Deserialize};

use serde_json::Result;

use url::{Url, ParseError};


#[derive(Serialize, Deserialize, Debug)]
struct Pokemon {
    name: String,
    national_pokedex_number: i16,
    poketype: Vec<[Option<PokeType>; 2]>,
    stats: PokeStats,
    image_url: String,
}

impl Pokemon {
}

#[derive(Serialize, Deserialize, Debug)]
enum PokeType {
    Fire,
    Flying,
    Grass,
    Poison,
    Water,
    
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
        poketype: vec!([Some(PokeType::Fire), Some(PokeType::Flying)]),
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

pub fn serialize_to_toml() -> std::io::Result<()> {
    // some data structure
    let charizard = Pokemon {
        name: "charizard".to_string(),
        national_pokedex_number: 6,
        poketype: vec!([Some(PokeType::Fire), Some(PokeType::Flying)]),
        stats: PokeStats {
            hp: 78,
            atk: 84,
            def: 78,
            spatk: 109,
            spdef: 85,
            spd: 100,
        },
        image_url: "https://archives.bulbagarden.net/media/upload/thumb/3/38/0006Charizard.png/375px-0006Charizard.png".to_string(),
    };

    let toml = toml::to_string(&charizard).unwrap();

    // https://serde.rs/convert-error.html
    let mut file = File::create("charizard.toml")?;
    file.write_all(toml.as_bytes());


    let charizard = Pokemon {
        name: "charizard".to_string(),
        national_pokedex_number: 6,
        poketype: vec!([Some(PokeType::Water), None]),
        stats: PokeStats {
            hp: 79,
            atk: 83,
            def: 100,
            spatk: 85,
            spdef: 105,
            spd: 78,
        },
        image_url: "https://archives.bulbagarden.net/media/upload/thumb/0/0a/0009Blastoise.png/375px-0009Blastoise.png".to_string(),
    };

    let toml = toml::to_string(&charizard).unwrap();

    // https://serde.rs/convert-error.html
    let mut file = File::create("src/pokerust/blastoise.toml")?; // ? is shorthand for match Ok(T), Err from a Result
    file.write_all(toml.as_bytes());

    Ok(())
}
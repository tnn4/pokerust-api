

fn main() {
    println!("Testing pokerust api.");
    pokerust_api::create_cat_db();
    pokerust_api::insert_and_select_cats();
    pokerust_api::pokerust::pokemon::create_json_by_serializing_data_structures();
}
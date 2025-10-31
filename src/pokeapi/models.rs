//! Models for the PokeApi that will be serialized and returned
//!  in server response.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Language {
    pub name: String,
}

#[derive(Deserialize)]
pub(crate) struct FlavorText {
    pub language: Language,
    pub flavor_text: String,
}

#[derive(Deserialize)]
pub struct Habitat {
    pub name: String,
}

#[derive(Deserialize)]
pub(crate) struct PokemonSpecies {
    pub is_legendary: bool,
    pub flavor_text_entries: Vec<FlavorText>,
    pub habitat: Habitat,
}

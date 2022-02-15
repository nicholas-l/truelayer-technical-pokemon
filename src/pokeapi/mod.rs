//! PokeAPI
//! This module provides a simple interface to `PokeAPI.co`.
//! It is setup to use a trait object to allow for mocking.

mod models;

use async_trait::async_trait;
use mockall::automock;
use reqwest::{StatusCode, Url};
use serde::Serialize;
use tracing::info;

use self::models::{FlavorText, PokemonSpecies};

#[derive(Serialize)]
pub struct Pokemon {
    pub name: String,
    pub description: String,
    pub habitat: String,
    pub is_legendary: bool,
}

impl Pokemon {
    fn new(name: String, description: String, habitat: String, is_legendary: bool) -> Pokemon {
        Pokemon {
            name,
            description,
            habitat,
            is_legendary,
        }
    }
}

#[derive(Debug)]
pub enum PokemonError {
    PokemonNotFound,
    FailedToFetchPokemonSpecies,
    FailedToFindDescription(String),
}

impl From<reqwest::Error> for PokemonError {
    fn from(e: reqwest::Error) -> Self {
        println!("{:?}", e);
        match e.status() {
            Some(StatusCode::NOT_FOUND) => PokemonError::PokemonNotFound,
            _ => PokemonError::FailedToFetchPokemonSpecies,
        }
    }
}

// Returns the first flavor text entry that matches the language.
fn get_description(entries: &[FlavorText], language: &str) -> Result<String, PokemonError> {
    let en_flavour_text = entries
        .iter()
        .find(|flavor_text| flavor_text.language.name == language)
        .ok_or_else(|| PokemonError::FailedToFindDescription(language.to_string()))?
        .flavor_text
        .clone();
    // The PokeApi returns the flavor text with newlines so we need to remove them.
    let description = en_flavour_text.replace('\n', " ");
    let description = description.replace('\u{000c}', " ");
    Ok(description)
}

/// Calls the PokeAPI and returns a Pokemon struct based on the name provided.

#[automock]
#[async_trait]
pub trait PokeApiTrait: Send + Sync {
    async fn get_information(&self, name: &str) -> Result<Pokemon, PokemonError>;
}

#[derive(Debug, Clone)]
pub struct PokeApi {
    endpoint: Url,
}

impl PokeApi {
    pub fn new(endpoint: Url) -> PokeApi {
        PokeApi { endpoint }
    }
}

#[async_trait]
impl PokeApiTrait for PokeApi {
    /// Calls the PokeAPI and returns a Pokemon struct based on the name provided.
    async fn get_information(&self, name: &str) -> Result<Pokemon, PokemonError> {
        let url = self
            .endpoint
            .join(&format!("pokemon-species/{}", name))
            .unwrap();
        info!("Fetching pokemon information from {}", url);
        // Call Pokeapi to get the pokemon information;
        let response = reqwest::get(url).await?;
        if response.status() != StatusCode::OK {
            return Err(PokemonError::PokemonNotFound);
        }

        let PokemonSpecies {
            is_legendary,
            flavor_text_entries,
            habitat,
        } = response.json().await?;

        let description = get_description(&flavor_text_entries, "en")?;

        Ok(Pokemon::new(
            name.to_string(),
            description,
            habitat.name,
            is_legendary,
        ))
    }
}

#[cfg(test)]
mod tests_description {
    use super::models::*;
    use super::*;

    #[test]
    fn test_get_description() {
        let entries = vec![
            FlavorText {
                flavor_text: "A legendary Pokémon".to_string(),
                language: Language {
                    name: "en".to_string(),
                },
            },
            FlavorText {
                flavor_text: "A rare Pokémon".to_string(),
                language: Language {
                    name: "en".to_string(),
                },
            },
        ];
        assert_eq!(
            get_description(&entries, "en").unwrap(),
            "A legendary Pokémon"
        );
    }

    #[test]
    fn test_get_description_not_found() {
        let entries = vec![FlavorText {
            flavor_text: "A legendary Pokémon".to_string(),
            language: Language {
                name: "fr".to_string(),
            },
        }];
        assert!(get_description(&entries, "en").is_err());
    }
}

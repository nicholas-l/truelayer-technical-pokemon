//! Handler module
//! This is the handlers of the API for the server.

use crate::{
    context::Context,
    funtranslation::{FunTranslationTrait, TranslationError, TranslationMethod},
    pokeapi::{PokeApiTrait, Pokemon, PokemonError},
};

#[derive(Debug)]
pub enum APIError {
    PokemonError(PokemonError),
    #[allow(dead_code)]
    // We deal with this in the handler however we are documenting it here for completeness.
    TranslationError(TranslationError),
}

impl From<PokemonError> for APIError {
    fn from(error: PokemonError) -> Self {
        APIError::PokemonError(error)
    }
}

impl From<TranslationError> for APIError {
    fn from(error: TranslationError) -> Self {
        APIError::TranslationError(error)
    }
}

/// A handler for returning the pokemon information using the PokeAPI.
pub async fn pokemon_information<P: PokeApiTrait, F: FunTranslationTrait>(
    context: Context<P, F>,
    name: String,
) -> Result<Pokemon, APIError> {
    context
        .pokeapi
        .get_information(&name)
        .await
        .map_err(APIError::from)
}

/// Given a pokemon name, returns the translated Pokemon description and other basic information using the following rules:
/// - If the pokemon's habitat is cave or its a lengendary Pokemon, then apply Yoda translation
/// - For all other Pokemon, apply the Shakespeare translation
/// - If unable to translate, then use the standard description
pub async fn pokemon_translated<P: PokeApiTrait, F: FunTranslationTrait>(
    context: Context<P, F>,
    name: String,
) -> Result<Pokemon, APIError> {
    let mut pokemon = context
        .pokeapi
        .get_information(&name)
        .await
        .map_err(APIError::from)?;
    let translation_method = if pokemon.is_legendary || pokemon.habitat == "cave" {
        TranslationMethod::Yoda
    } else {
        TranslationMethod::Shakespeare
    };

    let translated = context
        .fun_translation
        .translate(translation_method, &pokemon.description)
        .await;
    // If there was any problem in translating the text then use the standard description.
    pokemon.description = translated.unwrap_or(pokemon.description);
    Ok(pokemon)
}

#[cfg(test)]
mod test {
    use mockall::predicate;

    use crate::{funtranslation::MockFunTranslationTrait, pokeapi::MockPokeApiTrait};

    use super::*;

    #[tokio::test]
    async fn test_pokemon_translated() {
        let mut mock_pokeapi = MockPokeApiTrait::new();
        mock_pokeapi
            .expect_get_information()
            .with(predicate::eq("pikachu"))
            .returning(|_| {
                Ok(Pokemon {
                    name: "pikachu".to_string(),
                    description: "pika pika".to_string(),
                    habitat: "forest".to_string(),
                    is_legendary: true,
                })
            });

        let mut mock_translation = MockFunTranslationTrait::new();
        mock_translation
            .expect_translate()
            .with(
                predicate::eq(TranslationMethod::Yoda),
                predicate::eq("pika pika"),
            )
            .times(1)
            .returning(|_, _| Ok("translated text".to_string()));

        let context = Context::new(mock_pokeapi, mock_translation);

        let res = pokemon_translated(context, "pikachu".to_string()).await;

        assert!(res.is_ok());

        let pokemon = res.unwrap();

        assert_eq!(pokemon.name, "pikachu");
        assert_eq!(pokemon.description, "translated text");
    }
}

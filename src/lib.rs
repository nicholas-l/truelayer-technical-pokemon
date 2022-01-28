mod context;
mod funtranslation;
mod handlers;
mod pokeapi;

use handlers::APIError;
use serde::Serialize;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::reply::Json;
use warp::{reject, Filter, Rejection, Reply};

pub use context::Context;
pub use funtranslation::FunTranslation;
pub use pokeapi::PokeApi;
use pokeapi::PokemonError;

impl reject::Reject for APIError {}

pub(crate) fn with_context(
    context: Context<PokeApi, FunTranslation>,
) -> impl Filter<Extract = (Context<PokeApi, FunTranslation>,), Error = Infallible> + Clone {
    warp::any().map(move || context.clone())
}

pub fn pokemon_route(
    context: Context<PokeApi, FunTranslation>,
) -> impl Filter<Extract = (Json,), Error = Rejection> + Clone {
    warp::get()
        .and(with_context(context))
        .and(warp::path!("pokemon" / String))
        .and_then(|context, name| async {
            match handlers::pokemon_information::<PokeApi, FunTranslation>(context, name).await {
                Ok(pokemon) => Ok(warp::reply::json(&pokemon)),
                Err(err) => Err(reject::custom(err)),
            }
        })
}

pub fn pokemon_translated_route(
    context: Context<PokeApi, FunTranslation>,
) -> impl Filter<Extract = (Json,), Error = Rejection> + Clone {
    warp::get()
        .and(with_context(context))
        .and(warp::path!("pokemon" / "translated" / String))
        .and_then(
            |context: Context<PokeApi, FunTranslation>, name| async move {
                let context = context.clone();
                match handlers::pokemon_translated(context, name).await {
                    Ok(pokemon) => Ok(warp::reply::json(&pokemon)),
                    Err(err) => Err(reject::custom(err)),
                }
            },
        )
}

#[derive(Serialize)]
struct ErrorMessage {
    message: String,
}

/// A handler to recover from an error and return a status message to the consumer.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "NOT_FOUND")
    } else if let Some(err) = err.find::<APIError>() {
        match err {
            APIError::PokemonError(PokemonError::PokemonNotFound) => {
                (StatusCode::NOT_FOUND, "POKEMON_NOT_FOUND")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (StatusCode::METHOD_NOT_ALLOWED, "METHOD_NOT_ALLOWED")
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_REJECTION")
    };

    let json = warp::reply::json(&ErrorMessage {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

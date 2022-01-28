use reqwest::Url;
use truelayer_technical_pokemon::{
    handle_rejection, pokemon_route, pokemon_translated_route, Context, FunTranslation, PokeApi,
};

use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Create the tracing subscriber to log to stdout.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let pokeapi_endpoint = Url::parse("https://pokeapi.co/api/v2/").unwrap();
    let funtranslation_endpoint = Url::parse("https://api.funtranslations.com/translate/").unwrap();

    let pokemon_api = PokeApi::new(pokeapi_endpoint);
    let funtranslation = FunTranslation::new(funtranslation_endpoint);

    let context = Context::new(pokemon_api, funtranslation);

    // Only accept GET requests.
    let routes = pokemon_translated_route(context.clone())
        .or(pokemon_route(context.clone()))
        .recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

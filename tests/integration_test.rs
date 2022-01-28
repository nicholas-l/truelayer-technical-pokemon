use reqwest::Url;
use serde::Serialize;
use truelayer_technical_pokemon::{pokemon_route, Context, FunTranslation, PokeApi};

#[derive(Debug, Serialize)]
struct Message {
    message: String,
}

#[tokio::test]
async fn test_pokemon_information_not_found() {
    use httpmock::prelude::*;

    // Start a lightweight mock server.
    let server = MockServer::start();

    // Create a mock on the server.
    let pokeapi_mock = server.mock(|when, then| {
        when.method(GET).path("/pokemon-species/badpokemon");
        then.status(404)
            .header("content-type", "text/html")
            .body("Not Found");
    });

    let pokeapi_endpoint = &Url::parse(&format!("http://{}", server.address())).unwrap();
    println!("{}", pokeapi_endpoint);

    let response = warp::test::request()
        .path("/pokemon/badpokemon")
        .filter(&pokemon_route(Context::new(
            PokeApi::new(pokeapi_endpoint.clone()),
            FunTranslation::new(pokeapi_endpoint.clone()),
        )))
        .await;
    // Ensure the specified mock was called exactly one time (or fail with a detailed error description).
    pokeapi_mock.assert();

    assert!(response.is_err());
}

#[tokio::test]
async fn test_pokemon_information_found() {
    use httpmock::prelude::*;

    // Start a lightweight mock server.
    let server = MockServer::start();

    // Create a mock on the server.
    let pokeapi_mock = server.mock(|when, then| {
        when.method(GET).path("/pokemon-species/goodpokemon");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{
              "name":"goodpokemon",
              "is_legendary":false,
              "habitat": {
                "name": "grassland"
              },
              "flavor_text_entries": [
                {
                  "flavor_text": "When the bulb on\nits back grows\nlarge, it appears\fto lose the\nability to stand\non its hind legs.",
                  "language": {
                    "name": "en",
                    "url": "https://pokeapi.co/api/v2/language/9/"
                  },
                  "version": {
                    "name": "red",
                    "url": "https://pokeapi.co/api/v2/version/1/"
                  }
                }
              ]
            }"#);
      });

    let pokeapi_endpoint = &Url::parse(&format!("http://{}", server.address())).unwrap();

    let response = warp::test::request()
        .path("/pokemon/goodpokemon")
        .filter(&pokemon_route(Context::new(
            PokeApi::new(pokeapi_endpoint.clone()),
            FunTranslation::new(pokeapi_endpoint.clone()),
        )))
        .await;
    // Ensure the specified mock was called exactly one time (or fail with a detailed error description).
    pokeapi_mock.assert();

    assert!(response.is_ok());
}

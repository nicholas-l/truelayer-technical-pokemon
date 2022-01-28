use crate::{funtranslation::FunTranslationTrait, pokeapi::PokeApiTrait};

#[derive(Clone)]
pub struct Context<P: PokeApiTrait, F: FunTranslationTrait> {
    pub pokeapi: P,
    pub fun_translation: F,
}

impl<P: PokeApiTrait, F: FunTranslationTrait> Context<P, F> {
    pub fn new(pokeapi: P, fun_translation: F) -> Self {
        Context {
            pokeapi,
            fun_translation,
        }
    }
}

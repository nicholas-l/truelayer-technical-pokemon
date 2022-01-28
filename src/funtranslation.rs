//! Fun Translations
//! This encapsulates the API of funtranslations.com and provides a simple
//! interface to translate text.
use std::fmt::Display;

use async_trait::async_trait;
use mockall::automock;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize)]
struct Translation {
    success: TranslationResult,
    contents: TranslationContents,
}

#[derive(Deserialize)]
struct TranslationResult {
    total: usize,
}

#[derive(Deserialize)]
struct TranslationContents {
    translated: String,
}

/// Only two methods of translation are supported; Yoda and Shakespeare.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranslationMethod {
    Shakespeare,
    Yoda,
}

impl Display for TranslationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslationMethod::Shakespeare => write!(f, "shakespeare"),
            TranslationMethod::Yoda => write!(f, "yoda"),
        }
    }
}

#[derive(Debug)]
pub enum TranslationError {
    FailedToFetchTranslation,
    UnsuccessfulTranslation,
}

impl From<reqwest::Error> for TranslationError {
    fn from(_: reqwest::Error) -> Self {
        TranslationError::FailedToFetchTranslation
    }
}

#[automock]
#[async_trait]
pub trait FunTranslationTrait: Send + Sync {
    async fn translate(
        &self,
        method: TranslationMethod,
        text: &str,
    ) -> Result<String, TranslationError>;
}

#[derive(Clone)]
pub struct FunTranslation {
    endpoint: Url,
}

impl FunTranslation {
    pub fn new(endpoint: Url) -> Self {
        FunTranslation { endpoint }
    }
}

#[async_trait]
impl FunTranslationTrait for FunTranslation {
    async fn translate(
        &self,
        method: TranslationMethod,
        text: &str,
    ) -> Result<String, TranslationError> {
        let text = utf8_percent_encode(text, NON_ALPHANUMERIC);

        // This is safe to unwrap as the method value is controlled via the enum.
        let mut url = self.endpoint.join(&format!("{}.json", method)).unwrap();
        url.set_query(Some(&format!("text={}", text)));

        let Translation { success, contents } = reqwest::get(url).await?.json().await?;

        // If the translation was unsuccessful, return the original text
        if success.total == 0 {
            Err(TranslationError::UnsuccessfulTranslation)
        //Otherwise, return the translated text
        } else {
            Ok(contents.translated)
        }
    }
}

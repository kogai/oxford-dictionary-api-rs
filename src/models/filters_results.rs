/*
 *
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 1.11.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FiltersResults : A mapping of filters available per endpoints.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FiltersResults {
  /// A list of filters available for Retrieve Entry endpoint
  #[serde(rename = "entries")]
  entries: Option<Vec<String>>,
  /// A list of filters available for LEMMATRON endpoint
  #[serde(rename = "inflections")]
  inflections: Option<Vec<String>>,
  /// A list of filters available for Translations endpoint
  #[serde(rename = "translations")]
  translations: Option<Vec<String>>,
  /// A list of filters available for Translations endpoint
  #[serde(rename = "wordlist")]
  wordlist: Option<Vec<String>>,
}

impl FiltersResults {
  /// A mapping of filters available per endpoints.
  pub fn new() -> FiltersResults {
    FiltersResults {
      entries: None,
      inflections: None,
      translations: None,
      wordlist: None,
    }
  }

  pub fn set_entries(&mut self, entries: Vec<String>) {
    self.entries = Some(entries);
  }

  pub fn with_entries(mut self, entries: Vec<String>) -> FiltersResults {
    self.entries = Some(entries);
    self
  }

  pub fn entries(&self) -> Option<&Vec<String>> {
    self.entries.as_ref()
  }

  pub fn reset_entries(&mut self) {
    self.entries = None;
  }

  pub fn set_inflections(&mut self, inflections: Vec<String>) {
    self.inflections = Some(inflections);
  }

  pub fn with_inflections(mut self, inflections: Vec<String>) -> FiltersResults {
    self.inflections = Some(inflections);
    self
  }

  pub fn inflections(&self) -> Option<&Vec<String>> {
    self.inflections.as_ref()
  }

  pub fn reset_inflections(&mut self) {
    self.inflections = None;
  }

  pub fn set_translations(&mut self, translations: Vec<String>) {
    self.translations = Some(translations);
  }

  pub fn with_translations(mut self, translations: Vec<String>) -> FiltersResults {
    self.translations = Some(translations);
    self
  }

  pub fn translations(&self) -> Option<&Vec<String>> {
    self.translations.as_ref()
  }

  pub fn reset_translations(&mut self) {
    self.translations = None;
  }

  pub fn set_wordlist(&mut self, wordlist: Vec<String>) {
    self.wordlist = Some(wordlist);
  }

  pub fn with_wordlist(mut self, wordlist: Vec<String>) -> FiltersResults {
    self.wordlist = Some(wordlist);
    self
  }

  pub fn wordlist(&self) -> Option<&Vec<String>> {
    self.wordlist.as_ref()
  }

  pub fn reset_wordlist(&mut self) {
    self.wordlist = None;
  }
}

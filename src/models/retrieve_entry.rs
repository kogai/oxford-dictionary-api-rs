/*
 *
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 1.11.0
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RetrieveEntry : Schema for the 'entries' endpoints

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveEntry {
  /// Additional Information provided by OUP
  #[serde(rename = "metadata")]
  metadata: Option<Value>,
  /// A list of entries and all the data related to them
  #[serde(rename = "results")]
  results: Option<Vec<::models::HeadwordEntry>>,
}

impl RetrieveEntry {
  /// Schema for the 'entries' endpoints
  pub fn new() -> RetrieveEntry {
    RetrieveEntry {
      metadata: None,
      results: None,
    }
  }

  pub fn set_metadata(&mut self, metadata: Value) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: Value) -> RetrieveEntry {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&Value> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_results(&mut self, results: Vec<::models::HeadwordEntry>) {
    self.results = Some(results);
  }

  pub fn with_results(mut self, results: Vec<::models::HeadwordEntry>) -> RetrieveEntry {
    self.results = Some(results);
    self
  }

  pub fn results(&self) -> Option<&Vec<::models::HeadwordEntry>> {
    self.results.as_ref()
  }

  pub fn reset_results(&mut self) {
    self.results = None;
  }
}

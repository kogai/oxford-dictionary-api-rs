/* 
 * 
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 1.11.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamplesListInner {
  /// A list of statements of the exact meaning of a word
  #[serde(rename = "definitions")]
  definitions: Option<::models::Arrayofstrings>,
  /// A subject, discipline, or branch of knowledge particular to the Sense
  #[serde(rename = "domains")]
  domains: Option<::models::Arrayofstrings>,
  #[serde(rename = "notes")]
  notes: Option<::models::CategorizedTextList>,
  /// A particular area in which the pronunciation occurs, e.g. 'Great Britain'
  #[serde(rename = "regions")]
  regions: Option<::models::Arrayofstrings>,
  /// A level of language usage, typically with respect to formality. e.g. 'offensive', 'informal'
  #[serde(rename = "registers")]
  registers: Option<::models::Arrayofstrings>,
  /// The list of sense identifiers related to the example. Provided in the sentences endpoint only.
  #[serde(rename = "senseIds")]
  sense_ids: Option<::models::Arrayofstrings>,
  #[serde(rename = "text")]
  text: String,
  #[serde(rename = "translations")]
  translations: Option<::models::TranslationsList>
}

impl ExamplesListInner {
  pub fn new(text: String) -> ExamplesListInner {
    ExamplesListInner {
      definitions: None,
      domains: None,
      notes: None,
      regions: None,
      registers: None,
      sense_ids: None,
      text: text,
      translations: None
    }
  }

  pub fn set_definitions(&mut self, definitions: ::models::Arrayofstrings) {
    self.definitions = Some(definitions);
  }

  pub fn with_definitions(mut self, definitions: ::models::Arrayofstrings) -> ExamplesListInner {
    self.definitions = Some(definitions);
    self
  }

  pub fn definitions(&self) -> Option<&::models::Arrayofstrings> {
    self.definitions.as_ref()
  }

  pub fn reset_definitions(&mut self) {
    self.definitions = None;
  }

  pub fn set_domains(&mut self, domains: ::models::Arrayofstrings) {
    self.domains = Some(domains);
  }

  pub fn with_domains(mut self, domains: ::models::Arrayofstrings) -> ExamplesListInner {
    self.domains = Some(domains);
    self
  }

  pub fn domains(&self) -> Option<&::models::Arrayofstrings> {
    self.domains.as_ref()
  }

  pub fn reset_domains(&mut self) {
    self.domains = None;
  }

  pub fn set_notes(&mut self, notes: ::models::CategorizedTextList) {
    self.notes = Some(notes);
  }

  pub fn with_notes(mut self, notes: ::models::CategorizedTextList) -> ExamplesListInner {
    self.notes = Some(notes);
    self
  }

  pub fn notes(&self) -> Option<&::models::CategorizedTextList> {
    self.notes.as_ref()
  }

  pub fn reset_notes(&mut self) {
    self.notes = None;
  }

  pub fn set_regions(&mut self, regions: ::models::Arrayofstrings) {
    self.regions = Some(regions);
  }

  pub fn with_regions(mut self, regions: ::models::Arrayofstrings) -> ExamplesListInner {
    self.regions = Some(regions);
    self
  }

  pub fn regions(&self) -> Option<&::models::Arrayofstrings> {
    self.regions.as_ref()
  }

  pub fn reset_regions(&mut self) {
    self.regions = None;
  }

  pub fn set_registers(&mut self, registers: ::models::Arrayofstrings) {
    self.registers = Some(registers);
  }

  pub fn with_registers(mut self, registers: ::models::Arrayofstrings) -> ExamplesListInner {
    self.registers = Some(registers);
    self
  }

  pub fn registers(&self) -> Option<&::models::Arrayofstrings> {
    self.registers.as_ref()
  }

  pub fn reset_registers(&mut self) {
    self.registers = None;
  }

  pub fn set_sense_ids(&mut self, sense_ids: ::models::Arrayofstrings) {
    self.sense_ids = Some(sense_ids);
  }

  pub fn with_sense_ids(mut self, sense_ids: ::models::Arrayofstrings) -> ExamplesListInner {
    self.sense_ids = Some(sense_ids);
    self
  }

  pub fn sense_ids(&self) -> Option<&::models::Arrayofstrings> {
    self.sense_ids.as_ref()
  }

  pub fn reset_sense_ids(&mut self) {
    self.sense_ids = None;
  }

  pub fn set_text(&mut self, text: String) {
    self.text = text;
  }

  pub fn with_text(mut self, text: String) -> ExamplesListInner {
    self.text = text;
    self
  }

  pub fn text(&self) -> &String {
    &self.text
  }


  pub fn set_translations(&mut self, translations: ::models::TranslationsList) {
    self.translations = Some(translations);
  }

  pub fn with_translations(mut self, translations: ::models::TranslationsList) -> ExamplesListInner {
    self.translations = Some(translations);
    self
  }

  pub fn translations(&self) -> Option<&::models::TranslationsList> {
    self.translations.as_ref()
  }

  pub fn reset_translations(&mut self) {
    self.translations = None;
  }

}




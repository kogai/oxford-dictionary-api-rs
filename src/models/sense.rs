/* 
 * 
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 1.11.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Sense : A lexical sense represents the lexical meaning of a lexical entry when interpreted as referring to the corresponding ontology element

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sense {
  /// A grouping of crossreference notes.
  #[serde(rename = "crossReferenceMarkers")]
  cross_reference_markers: Option<::models::Arrayofstrings>,
  #[serde(rename = "crossReferences")]
  cross_references: Option<::models::CrossReferencesList>,
  /// A list of statements of the exact meaning of a word
  #[serde(rename = "definitions")]
  definitions: Option<::models::Arrayofstrings>,
  /// A subject, discipline, or branch of knowledge particular to the Sense
  #[serde(rename = "domains")]
  domains: Option<::models::Arrayofstrings>,
  #[serde(rename = "examples")]
  examples: Option<::models::ExamplesList>,
  /// The id of the sense that is required for the delete procedure
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "notes")]
  notes: Option<::models::CategorizedTextList>,
  #[serde(rename = "pronunciations")]
  pronunciations: Option<::models::PronunciationsList>,
  /// A particular area in which the Sense occurs, e.g. 'Great Britain'
  #[serde(rename = "regions")]
  regions: Option<::models::Arrayofstrings>,
  /// A level of language usage, typically with respect to formality. e.g. 'offensive', 'informal'
  #[serde(rename = "registers")]
  registers: Option<::models::Arrayofstrings>,
  /// A list of short statements of the exact meaning of a word
  #[serde(rename = "short_definitions")]
  short_definitions: Option<::models::Arrayofstrings>,
  /// Ordered list of subsenses of a sense
  #[serde(rename = "subsenses")]
  subsenses: Option<Vec<::models::Sense>>,
  /// Ordered list of links to the Thesaurus Dictionary
  #[serde(rename = "thesaurusLinks")]
  thesaurus_links: Option<Vec<::models::ThesaurusLink>>,
  #[serde(rename = "translations")]
  translations: Option<::models::TranslationsList>,
  /// Various words that are used interchangeably depending on the context, e.g 'duck' and 'duck boat'
  #[serde(rename = "variantForms")]
  variant_forms: Option<::models::VariantFormsList>
}

impl Sense {
  /// A lexical sense represents the lexical meaning of a lexical entry when interpreted as referring to the corresponding ontology element
  pub fn new() -> Sense {
    Sense {
      cross_reference_markers: None,
      cross_references: None,
      definitions: None,
      domains: None,
      examples: None,
      id: None,
      notes: None,
      pronunciations: None,
      regions: None,
      registers: None,
      short_definitions: None,
      subsenses: None,
      thesaurus_links: None,
      translations: None,
      variant_forms: None
    }
  }

  pub fn set_cross_reference_markers(&mut self, cross_reference_markers: ::models::Arrayofstrings) {
    self.cross_reference_markers = Some(cross_reference_markers);
  }

  pub fn with_cross_reference_markers(mut self, cross_reference_markers: ::models::Arrayofstrings) -> Sense {
    self.cross_reference_markers = Some(cross_reference_markers);
    self
  }

  pub fn cross_reference_markers(&self) -> Option<&::models::Arrayofstrings> {
    self.cross_reference_markers.as_ref()
  }

  pub fn reset_cross_reference_markers(&mut self) {
    self.cross_reference_markers = None;
  }

  pub fn set_cross_references(&mut self, cross_references: ::models::CrossReferencesList) {
    self.cross_references = Some(cross_references);
  }

  pub fn with_cross_references(mut self, cross_references: ::models::CrossReferencesList) -> Sense {
    self.cross_references = Some(cross_references);
    self
  }

  pub fn cross_references(&self) -> Option<&::models::CrossReferencesList> {
    self.cross_references.as_ref()
  }

  pub fn reset_cross_references(&mut self) {
    self.cross_references = None;
  }

  pub fn set_definitions(&mut self, definitions: ::models::Arrayofstrings) {
    self.definitions = Some(definitions);
  }

  pub fn with_definitions(mut self, definitions: ::models::Arrayofstrings) -> Sense {
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

  pub fn with_domains(mut self, domains: ::models::Arrayofstrings) -> Sense {
    self.domains = Some(domains);
    self
  }

  pub fn domains(&self) -> Option<&::models::Arrayofstrings> {
    self.domains.as_ref()
  }

  pub fn reset_domains(&mut self) {
    self.domains = None;
  }

  pub fn set_examples(&mut self, examples: ::models::ExamplesList) {
    self.examples = Some(examples);
  }

  pub fn with_examples(mut self, examples: ::models::ExamplesList) -> Sense {
    self.examples = Some(examples);
    self
  }

  pub fn examples(&self) -> Option<&::models::ExamplesList> {
    self.examples.as_ref()
  }

  pub fn reset_examples(&mut self) {
    self.examples = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> Sense {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_notes(&mut self, notes: ::models::CategorizedTextList) {
    self.notes = Some(notes);
  }

  pub fn with_notes(mut self, notes: ::models::CategorizedTextList) -> Sense {
    self.notes = Some(notes);
    self
  }

  pub fn notes(&self) -> Option<&::models::CategorizedTextList> {
    self.notes.as_ref()
  }

  pub fn reset_notes(&mut self) {
    self.notes = None;
  }

  pub fn set_pronunciations(&mut self, pronunciations: ::models::PronunciationsList) {
    self.pronunciations = Some(pronunciations);
  }

  pub fn with_pronunciations(mut self, pronunciations: ::models::PronunciationsList) -> Sense {
    self.pronunciations = Some(pronunciations);
    self
  }

  pub fn pronunciations(&self) -> Option<&::models::PronunciationsList> {
    self.pronunciations.as_ref()
  }

  pub fn reset_pronunciations(&mut self) {
    self.pronunciations = None;
  }

  pub fn set_regions(&mut self, regions: ::models::Arrayofstrings) {
    self.regions = Some(regions);
  }

  pub fn with_regions(mut self, regions: ::models::Arrayofstrings) -> Sense {
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

  pub fn with_registers(mut self, registers: ::models::Arrayofstrings) -> Sense {
    self.registers = Some(registers);
    self
  }

  pub fn registers(&self) -> Option<&::models::Arrayofstrings> {
    self.registers.as_ref()
  }

  pub fn reset_registers(&mut self) {
    self.registers = None;
  }

  pub fn set_short_definitions(&mut self, short_definitions: ::models::Arrayofstrings) {
    self.short_definitions = Some(short_definitions);
  }

  pub fn with_short_definitions(mut self, short_definitions: ::models::Arrayofstrings) -> Sense {
    self.short_definitions = Some(short_definitions);
    self
  }

  pub fn short_definitions(&self) -> Option<&::models::Arrayofstrings> {
    self.short_definitions.as_ref()
  }

  pub fn reset_short_definitions(&mut self) {
    self.short_definitions = None;
  }

  pub fn set_subsenses(&mut self, subsenses: Vec<::models::Sense>) {
    self.subsenses = Some(subsenses);
  }

  pub fn with_subsenses(mut self, subsenses: Vec<::models::Sense>) -> Sense {
    self.subsenses = Some(subsenses);
    self
  }

  pub fn subsenses(&self) -> Option<&Vec<::models::Sense>> {
    self.subsenses.as_ref()
  }

  pub fn reset_subsenses(&mut self) {
    self.subsenses = None;
  }

  pub fn set_thesaurus_links(&mut self, thesaurus_links: Vec<::models::ThesaurusLink>) {
    self.thesaurus_links = Some(thesaurus_links);
  }

  pub fn with_thesaurus_links(mut self, thesaurus_links: Vec<::models::ThesaurusLink>) -> Sense {
    self.thesaurus_links = Some(thesaurus_links);
    self
  }

  pub fn thesaurus_links(&self) -> Option<&Vec<::models::ThesaurusLink>> {
    self.thesaurus_links.as_ref()
  }

  pub fn reset_thesaurus_links(&mut self) {
    self.thesaurus_links = None;
  }

  pub fn set_translations(&mut self, translations: ::models::TranslationsList) {
    self.translations = Some(translations);
  }

  pub fn with_translations(mut self, translations: ::models::TranslationsList) -> Sense {
    self.translations = Some(translations);
    self
  }

  pub fn translations(&self) -> Option<&::models::TranslationsList> {
    self.translations.as_ref()
  }

  pub fn reset_translations(&mut self) {
    self.translations = None;
  }

  pub fn set_variant_forms(&mut self, variant_forms: ::models::VariantFormsList) {
    self.variant_forms = Some(variant_forms);
  }

  pub fn with_variant_forms(mut self, variant_forms: ::models::VariantFormsList) -> Sense {
    self.variant_forms = Some(variant_forms);
    self
  }

  pub fn variant_forms(&self) -> Option<&::models::VariantFormsList> {
    self.variant_forms.as_ref()
  }

  pub fn reset_variant_forms(&mut self) {
    self.variant_forms = None;
  }

}




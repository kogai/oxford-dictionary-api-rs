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
pub struct InflectionsListInner {
  /// The identifier of the word
  #[serde(rename = "id")]
  id: String,
  #[serde(rename = "text")]
  text: String
}

impl InflectionsListInner {
  pub fn new(id: String, text: String) -> InflectionsListInner {
    InflectionsListInner {
      id: id,
      text: text
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }

  pub fn with_id(mut self, id: String) -> InflectionsListInner {
    self.id = id;
    self
  }

  pub fn id(&self) -> &String {
    &self.id
  }


  pub fn set_text(&mut self, text: String) {
    self.text = text;
  }

  pub fn with_text(mut self, text: String) -> InflectionsListInner {
    self.text = text;
    self
  }

  pub fn text(&self) -> &String {
    &self.text
  }


}




/* 
 * 
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 1.11.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use hyper::header::UserAgent;

use super::{Error, configuration};

pub struct LexiStatsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> LexiStatsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> LexiStatsApiClient<C> {
        LexiStatsApiClient {
            configuration: configuration,
        }
    }
}

pub trait LexiStatsApi {
    fn stats_frequency_ngrams_source_lang_corpus_ngram_size_get(&self, source_lang: &str, corpus: &str, ngram_size: &str, app_id: &str, app_key: &str, tokens: &str, contains: &str, punctuation: &str, format: &str, min_frequency: i64, max_frequency: i64, min_document_frequency: i64, max_document_frequency: i64, collate: &str, sort: &str, offset: i64, limit: i64) -> Box<Future<Item = ::models::NgramsResult, Error = Error<serde_json::Value>>>;
    fn stats_frequency_word_source_lang_get(&self, source_lang: &str, app_id: &str, app_key: &str, corpus: &str, wordform: &str, true_case: &str, lemma: &str, lexical_category: &str) -> Box<Future<Item = ::models::StatsWordResult, Error = Error<serde_json::Value>>>;
    fn stats_frequency_words_source_lang_get(&self, source_lang: &str, app_id: &str, app_key: &str, corpus: &str, wordform: &str, true_case: &str, lemma: &str, lexical_category: &str, grammatical_features: &str, sort: &str, collate: &str, min_frequency: i64, max_frequency: i64, min_normalized_frequency: f32, max_normalized_frequency: f32, offset: i64, limit: i64) -> Box<Future<Item = ::models::StatsWordResultList, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>LexiStatsApi for LexiStatsApiClient<C> {
    fn stats_frequency_ngrams_source_lang_corpus_ngram_size_get(&self, source_lang: &str, corpus: &str, ngram_size: &str, app_id: &str, app_key: &str, tokens: &str, contains: &str, punctuation: &str, format: &str, min_frequency: i64, max_frequency: i64, min_document_frequency: i64, max_document_frequency: i64, collate: &str, sort: &str, offset: i64, limit: i64) -> Box<Future<Item = ::models::NgramsResult, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("tokens", &tokens.to_string());
            query.append_pair("contains", &contains.to_string());
            query.append_pair("punctuation", &punctuation.to_string());
            query.append_pair("format", &format.to_string());
            query.append_pair("minFrequency", &min_frequency.to_string());
            query.append_pair("maxFrequency", &max_frequency.to_string());
            query.append_pair("minDocumentFrequency", &min_document_frequency.to_string());
            query.append_pair("maxDocumentFrequency", &max_document_frequency.to_string());
            query.append_pair("collate", &collate.to_string());
            query.append_pair("sort", &sort.to_string());
            query.append_pair("offset", &offset.to_string());
            query.append_pair("limit", &limit.to_string());
            query.finish()
        };
        let uri_str = format!("{}/stats/frequency/ngrams/{source_lang}/{corpus}/{ngram-size}/?{}", configuration.base_path, query_string, source_lang=source_lang, corpus=corpus, ngram-size=ngram_size);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("app_id", app_id);
            headers.set_raw("app_key", app_key);
        }



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::NgramsResult, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn stats_frequency_word_source_lang_get(&self, source_lang: &str, app_id: &str, app_key: &str, corpus: &str, wordform: &str, true_case: &str, lemma: &str, lexical_category: &str) -> Box<Future<Item = ::models::StatsWordResult, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("corpus", &corpus.to_string());
            query.append_pair("wordform", &wordform.to_string());
            query.append_pair("trueCase", &true_case.to_string());
            query.append_pair("lemma", &lemma.to_string());
            query.append_pair("lexicalCategory", &lexical_category.to_string());
            query.finish()
        };
        let uri_str = format!("{}/stats/frequency/word/{source_lang}/?{}", configuration.base_path, query_string, source_lang=source_lang);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("app_id", app_id);
            headers.set_raw("app_key", app_key);
        }



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::StatsWordResult, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn stats_frequency_words_source_lang_get(&self, source_lang: &str, app_id: &str, app_key: &str, corpus: &str, wordform: &str, true_case: &str, lemma: &str, lexical_category: &str, grammatical_features: &str, sort: &str, collate: &str, min_frequency: i64, max_frequency: i64, min_normalized_frequency: f32, max_normalized_frequency: f32, offset: i64, limit: i64) -> Box<Future<Item = ::models::StatsWordResultList, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("corpus", &corpus.to_string());
            query.append_pair("wordform", &wordform.to_string());
            query.append_pair("trueCase", &true_case.to_string());
            query.append_pair("lemma", &lemma.to_string());
            query.append_pair("lexicalCategory", &lexical_category.to_string());
            query.append_pair("grammaticalFeatures", &grammatical_features.to_string());
            query.append_pair("sort", &sort.to_string());
            query.append_pair("collate", &collate.to_string());
            query.append_pair("minFrequency", &min_frequency.to_string());
            query.append_pair("maxFrequency", &max_frequency.to_string());
            query.append_pair("minNormalizedFrequency", &min_normalized_frequency.to_string());
            query.append_pair("maxNormalizedFrequency", &max_normalized_frequency.to_string());
            query.append_pair("offset", &offset.to_string());
            query.append_pair("limit", &limit.to_string());
            query.finish()
        };
        let uri_str = format!("{}/stats/frequency/words/{source_lang}/?{}", configuration.base_path, query_string, source_lang=source_lang);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("app_id", app_id);
            headers.set_raw("app_key", app_key);
        }



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::StatsWordResultList, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}

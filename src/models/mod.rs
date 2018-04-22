mod array_of_related_entries;
pub use self::array_of_related_entries::ArrayOfRelatedEntries;
mod array_of_related_entries_inner;
pub use self::array_of_related_entries_inner::ArrayOfRelatedEntriesInner;
mod arrayofstrings;
pub use self::arrayofstrings::Arrayofstrings;
mod categorized_text_list;
pub use self::categorized_text_list::CategorizedTextList;
mod categorized_text_list_inner;
pub use self::categorized_text_list_inner::CategorizedTextListInner;
mod cross_references_list;
pub use self::cross_references_list::CrossReferencesList;
mod cross_references_list_inner;
pub use self::cross_references_list_inner::CrossReferencesListInner;
mod derivative;
pub use self::derivative::Derivative;
mod entry;
pub use self::entry::Entry;
mod examples_list;
pub use self::examples_list::ExamplesList;
mod examples_list_inner;
pub use self::examples_list_inner::ExamplesListInner;
mod filters;
pub use self::filters::Filters;
mod filters_results;
pub use self::filters_results::FiltersResults;
mod grammatical_features_list;
pub use self::grammatical_features_list::GrammaticalFeaturesList;
mod grammatical_features_list_inner;
pub use self::grammatical_features_list_inner::GrammaticalFeaturesListInner;
mod headword_entry;
pub use self::headword_entry::HeadwordEntry;
mod headword_lemmatron;
pub use self::headword_lemmatron::HeadwordLemmatron;
mod headword_thesaurus;
pub use self::headword_thesaurus::HeadwordThesaurus;
mod inflections_list;
pub use self::inflections_list::InflectionsList;
mod inflections_list_inner;
pub use self::inflections_list_inner::InflectionsListInner;
mod languages;
pub use self::languages::Languages;
mod languages_results;
pub use self::languages_results::LanguagesResults;
mod languages_source_language;
pub use self::languages_source_language::LanguagesSourceLanguage;
mod languages_target_language;
pub use self::languages_target_language::LanguagesTargetLanguage;
mod lemmatron;
pub use self::lemmatron::Lemmatron;
mod lemmatron_lexical_entry;
pub use self::lemmatron_lexical_entry::LemmatronLexicalEntry;
mod lexical_entry;
pub use self::lexical_entry::LexicalEntry;
mod ngrams_result;
pub use self::ngrams_result::NgramsResult;
mod ngrams_result_results;
pub use self::ngrams_result_results::NgramsResultResults;
mod pronunciations_list;
pub use self::pronunciations_list::PronunciationsList;
mod pronunciations_list_inner;
pub use self::pronunciations_list_inner::PronunciationsListInner;
mod regions;
pub use self::regions::Regions;
mod retrieve_entry;
pub use self::retrieve_entry::RetrieveEntry;
mod sense;
pub use self::sense::Sense;
mod sentences_entry;
pub use self::sentences_entry::SentencesEntry;
mod sentences_lexical_entry;
pub use self::sentences_lexical_entry::SentencesLexicalEntry;
mod sentences_results;
pub use self::sentences_results::SentencesResults;
mod stats_word_result;
pub use self::stats_word_result::StatsWordResult;
mod stats_word_result_list;
pub use self::stats_word_result_list::StatsWordResultList;
mod stats_word_result_list_results;
pub use self::stats_word_result_list_results::StatsWordResultListResults;
mod stats_word_result_result;
pub use self::stats_word_result_result::StatsWordResultResult;
mod synonyms_antonyms;
pub use self::synonyms_antonyms::SynonymsAntonyms;
mod synonyms_antonyms_inner;
pub use self::synonyms_antonyms_inner::SynonymsAntonymsInner;
mod thesaurus;
pub use self::thesaurus::Thesaurus;
mod thesaurus_entry;
pub use self::thesaurus_entry::ThesaurusEntry;
mod thesaurus_lexical_entry;
pub use self::thesaurus_lexical_entry::ThesaurusLexicalEntry;
mod thesaurus_link;
pub use self::thesaurus_link::ThesaurusLink;
mod thesaurus_sense;
pub use self::thesaurus_sense::ThesaurusSense;
mod translations_list;
pub use self::translations_list::TranslationsList;
mod translations_list_inner;
pub use self::translations_list_inner::TranslationsListInner;
mod utility_labels;
pub use self::utility_labels::UtilityLabels;
mod utility_labels_results;
pub use self::utility_labels_results::UtilityLabelsResults;
mod variant_forms_list;
pub use self::variant_forms_list::VariantFormsList;
mod variant_forms_list_inner;
pub use self::variant_forms_list_inner::VariantFormsListInner;
mod wordlist;
pub use self::wordlist::Wordlist;
mod wordlist_results;
pub use self::wordlist_results::WordlistResults;

// TODO(farcaller): sort out files
pub struct File;

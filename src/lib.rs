use std::fs;
use std::path::PathBuf;
use indexmap::{IndexMap, indexmap};
use tree_sitter::{Parser};
use tree_sitter_tags::{TagsConfiguration, TagsContext};

#[allow(dead_code)]
fn get_parser() -> Parser {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_javascript::language()).expect("Error loading JavaScript grammar");
    parser
}

// Open source file
#[allow(dead_code)]
fn read_fixture(fixture_name: &str) -> Vec<u8> {
    let file_path = fs::canonicalize(PathBuf::from(format!("./fixtures/{}", fixture_name)));
    fs::read(file_path.unwrap()).expect("Can't open fixture file")
}

// Parse tags from supplied source.
#[allow(dead_code)]
fn parse_tags<'a>(config: &'a TagsConfiguration, source: &'a [u8]) -> Vec<IndexMap<&'a str, &'a str>> {
    let mut context = TagsContext::new();

    let tags = context.generate_tags(config, source, None).expect("Can't parse source code").0;

    let mut parsed: Vec<IndexMap<&str, &str>> = vec![];
    for result in tags {
        let tag = result.unwrap();

        let name = std::str::from_utf8(&source[tag.name_range]).unwrap_or("");
        let kind = config.syntax_type_name(tag.syntax_type_id);
        let def_or_ref = if tag.is_definition { "def" } else { "ref" };
        let first_line = std::str::from_utf8(&source[tag.line_range]).unwrap_or("");

        parsed.push(indexmap!{
                "name" => name,
                "kind" => kind,
                "def_or_ref" => def_or_ref,
                "first_line" => first_line,
            });
    }

    parsed
}

#[cfg(test)]
mod tests {
    use indexmap::{indexmap, IndexMap};
    use super::*;

    #[test]
    fn it_should_allow_js() {
        let source = read_fixture("Animal.js");

        let config = TagsConfiguration::new(
            tree_sitter_javascript::language(),
            tree_sitter_javascript::TAGGING_QUERY,
            tree_sitter_javascript::LOCALS_QUERY,
        ).unwrap();

        let tags = parse_tags(&config, &source);

        let expected: Vec<IndexMap<&str, &str>> = vec![
            indexmap! {"name" => "Animal", "kind" => "class", "def_or_ref" => "def", "first_line" => "class Animal extends Model {"},
            indexmap! {"name" => "tableName", "kind" => "method", "def_or_ref" => "def", "first_line" => "static get tableName() {"},
            indexmap! {"name" => "jsonSchema", "kind" => "method", "def_or_ref" => "def", "first_line" => "static get jsonSchema() {"},
            indexmap! {"name" => "relationMappings", "kind" => "method", "def_or_ref" => "def", "first_line" => "static get relationMappings() {"}
        ];

        assert_eq!(tags, expected);
    }
}
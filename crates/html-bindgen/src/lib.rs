use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

mod generate;
mod parse;
mod types;

use generate::def_to_string;
use scraper::ElementRef;
use types::*;

pub type Database = HashSet<Definition>;

pub fn generate_html(path: &Path) -> types::Result<String> {
    let database = parse(path)?;

    let mut output = String::new();
    output.push_str(
        "/// An HTML Element
        pub trait HtmlElement: ::std::fmt::Display {}\n
        ",
    );
    for entry in database {
        output.push_str(&def_to_string(entry));
    }

    // missing types in the IDL files!
    output.push_str(
        "#[derive(Default, Debug, PartialEq, Clone)]
        pub struct FileList {}
        
        #[derive(Default, Debug, PartialEq, Clone)]
        pub struct EventTarget {}
        ",
    );
    Ok(output)
}

pub fn parse(path: &Path) -> types::Result<Database> {
    let iter = fs::read_dir(path)?
        .into_iter()
        .map(|path| fs::read_to_string(path?.path()));
    let database = parse::parse_webidl(iter)?;

    Ok(database)
}

/// Parse the WhatWG HTML standards document.
///
/// # Design
///
/// The entire HTML spec is a flat document with little hierarchy. we first need to find
/// the metadata section labeled by `.element`. Then we need to track back through the
/// siblings to find the first `h4` node. That will contain the title of the elements.
///
/// Once we have the title, we can inspect the `.element` node properly. This is a nested
/// table containing strings. We then parse these strings into a structured representation.
pub fn parse_spec(spec: String) -> types::Result<()> {
    let document = scraper::Html::parse_document(&spec);
    let selector = scraper::Selector::parse(".element").unwrap();

    let extract_text = |child| {
        let categories = ElementRef::wrap(child).unwrap();
        categories.text().collect::<String>()
    };

    let mut specs = vec![];

    // TODO: make this take all elements
    for element in document.select(&selector).into_iter().take(10) {
        let tag_names = extract_tag_names(element);

        let mut current: Option<(String, Vec<String>)> = None;
        let mut outputs: HashMap<String, Vec<String>> = HashMap::new();

        for child in element.children() {
            let el = child.value().as_element();
            let tag_name = el.as_deref().unwrap().name();
            match tag_name {
                "dt" => {
                    if current.is_some() {
                        let current = current.take().unwrap();
                        outputs.insert(current.0, current.1);
                    }
                    current = Some((extract_text(child), vec![]));
                }
                "dd" => {
                    // dbg!(&outputs);
                    let current = current.as_mut().unwrap();
                    current.1.push(extract_text(child));
                }
                other => panic!("unexpected tag name {other}"),
            }
        }
        if current.is_some() {
            let current = current.take().unwrap();
            outputs.insert(current.0, current.1);
        }

        // children.next().unwrap();
        // let categories = extract_text(children.next().unwrap());

        // children.next().unwrap();
        // let contexts = extract_text(children.next().unwrap());

        // children.next().unwrap();
        // children.next().unwrap();
        // let content_model = extract_text(children.next().unwrap());

        // children.next().unwrap();
        // let tag_omission = extract_text(children.next().unwrap());

        // children.next().unwrap();
        // let content_attributes = extract_text(children.next().unwrap());

        // children.next().unwrap();
        // let dom_interface = extract_text(children.next().unwrap());

        for tag_name in tag_names {
            specs.push(RawSpec {
                tag_name,
                categories: outputs.get("Categories:").as_deref().unwrap().clone(),
                contexts: outputs
                    .get("Contexts in which this element can be used:")
                    .as_deref()
                    .unwrap()
                    .clone(),
                content_model: outputs.get("Content model:").as_deref().unwrap().clone(),
                content_attributes: outputs
                    .get("Content attributes:")
                    .as_deref()
                    .unwrap()
                    .clone(),
                tag_omission: outputs
                    .get("Tag omission in text/html:")
                    .as_deref()
                    .unwrap()
                    .clone(),
                // TODO: fix this one, we're not yet getting all data!
                dom_interface: outputs.get("DOM interface:").as_deref().unwrap().clone(),
            });
        }
    }
    dbg!(specs);
    Ok(())
}

/// The raw values extracted from the HTML spec
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct RawSpec {
    tag_name: String,
    categories: Vec<String>,
    contexts: Vec<String>,
    content_model: Vec<String>,
    tag_omission: Vec<String>,
    content_attributes: Vec<String>,
    dom_interface: Vec<String>,
}

/// Extract the tag names from the document.
fn extract_tag_names(element: scraper::ElementRef) -> Vec<String> {
    // Find the name of the element we're inspecting.
    let mut sibling = element.prev_sibling().unwrap();
    loop {
        if let scraper::node::Node::Element(element) = sibling.value() {
            if element.name() == "h4" {
                let s = element.id.as_ref().expect("could not parse h4 element id");
                return parse_tag_names(s);
            }
        }

        sibling = sibling.prev_sibling().unwrap();
    }
}

/// Parse the HTML tag names.
///
/// A single HTML heading can correspond to several HTML nodes
fn parse_tag_names(s: &str) -> Vec<String> {
    if s.ends_with("elements") {
        let iter = s
            .strip_prefix("the-")
            .unwrap()
            .strip_suffix("-elements")
            .unwrap()
            .split(",-");
        iter.map(|name| match name.strip_prefix("and-") {
            Some(name) => name.to_owned(),
            None => name.to_owned(),
        })
        .collect()
    } else {
        let s = s
            .strip_prefix("the-")
            .unwrap()
            .strip_suffix("-element")
            .unwrap()
            .to_owned();
        vec![s]
    }
}

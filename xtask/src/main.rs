use std::{env::current_dir, fs};

use async_std::io::WriteExt;
use html_bindgen::generate::{CodeFile, Module};
use html_bindgen::parse::{Attribute, ParsedElement};
use html_bindgen::scrape::ScrapedElement;
use structopt::StructOpt;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, Error>;

const HTML_STANDARD_URL: &str = "https://html.spec.whatwg.org";
const HTML_STANDARD_PATH: &str = "resources/standards/html.html";
const ARIA_STANDARD_URL: &str = "https://w3c.github.io/html-aria/";
const ARIA_STANDARD_PATH: &str = "resources/standards/aria.html";
const SCRAPED_ELEMENTS_PATH: &str = "resources/scraped/elements";
const PARSED_ELEMENTS_PATH: &str = "resources/parsed/elements";
const HTML_SYS_CRATE_PATH: &str = "crates/html-sys/src";
const HTML_CRATE_ELEMENTS_PATH: &str = "crates/html/src/generated";
const MANUAL_PATH: &str = "resources/manual";

/// Tooling for the Rust `html` crate
#[derive(StructOpt)]
enum Opt {
    /// Fetch, scrape, parse, and generate bindings
    All,
    /// Fetch the latest copies of the HTML standards
    Fetch,
    /// Scrape the raw standards into structured data
    Scrape,
    /// Parse the structured standards data into normalized form
    Parse,
    /// Generate Rust source code from the parsed data
    Generate,
}

#[async_std::main]
async fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::All => all().await?,
        Opt::Generate => {
            generate_sys()?;
            generate_html()?;
        }
        Opt::Scrape => scrape()?,
        Opt::Parse => parse()?,
        Opt::Fetch => fetch().await?,
    }
    Ok(())
}

async fn all() -> Result<()> {
    fetch().await?;
    scrape()?;
    generate_sys()?;
    generate_html()?;
    Ok(())
}

async fn fetch() -> Result<()> {
    async fn fetch(from: &str, to: &str) -> Result<()> {
        eprintln!("fetching: {from}");
        let body = surf::get(from).recv_string().await?;
        let mut target = async_std::fs::File::create(to).await?;
        target.write_all(body.as_bytes()).await?;
        eprintln!("updated: {to}");
        Ok(())
    }
    eprintln!("task: fetch");
    fetch(HTML_STANDARD_URL, HTML_STANDARD_PATH).await?;
    fetch(ARIA_STANDARD_URL, ARIA_STANDARD_PATH).await?;
    Ok(())
}

fn scrape() -> Result<()> {
    let spec = fs::read_to_string(current_dir()?.join(HTML_STANDARD_PATH))?;
    let nodes = html_bindgen::scrape::scrape_spec(spec)?
        .into_iter()
        .map(|n| (n.tag_name.clone(), n));
    persist_nodes(nodes, SCRAPED_ELEMENTS_PATH)?;
    Ok(())
}

fn parse() -> Result<()> {
    eprintln!("task: parse");
    let iter = lookup_nodes::<ScrapedElement>(SCRAPED_ELEMENTS_PATH)?;
    let nodes = html_bindgen::parse::parse(iter)?
        .into_iter()
        .map(|n| (n.tag_name.clone(), n));
    persist_nodes(nodes, PARSED_ELEMENTS_PATH)?;
    Ok(())
}

fn generate_sys() -> Result<()> {
    eprintln!("task: generate sys");
    let parsed = lookup_nodes::<ParsedElement>(PARSED_ELEMENTS_PATH)?;
    let manual = lookup_file::<Vec<Attribute>>(MANUAL_PATH, "global_attributes")?;
    let modules = lookup_file::<Vec<Module>>(MANUAL_PATH, "web_sys_modules")?;
    let nodes = html_bindgen::generate::sys::generate(parsed, &manual, &modules)?;

    let root_dir = current_dir()?.join(HTML_SYS_CRATE_PATH);
    let _ = fs::remove_dir_all(&root_dir);
    for code in nodes {
        let dir = root_dir.join(&code.dir);
        fs::create_dir_all(&dir)?;

        let filename = dir.join(&code.filename);
        eprintln!(
            "writing: {}/{}/{}",
            HTML_SYS_CRATE_PATH, code.dir, code.filename
        );
        std::fs::write(filename, code.code.as_bytes())?;
    }
    Ok(())
}

fn generate_html() -> Result<()> {
    eprintln!("task: generate html");
    let parsed = lookup_nodes::<ParsedElement>(PARSED_ELEMENTS_PATH)?;
    let manual = lookup_file::<Vec<Attribute>>(MANUAL_PATH, "global_attributes")?;
    let modules = lookup_file::<Vec<Module>>(MANUAL_PATH, "web_sys_modules")?;
    let nodes = html_bindgen::generate::html::generate(parsed, &manual, modules.as_slice())?;

    let root_dir = current_dir()?.join(HTML_CRATE_ELEMENTS_PATH);
    let _ = fs::remove_dir_all(&root_dir);
    for code in nodes {
        let dir = root_dir.join(&code.dir);
        fs::create_dir_all(&dir)?;

        let filename = dir.join(&code.filename);
        eprintln!(
            "writing: {}/{}/{}",
            HTML_CRATE_ELEMENTS_PATH, code.dir, code.filename
        );
        std::fs::write(filename, code.code.as_bytes())?;
    }
    Ok(())
}

fn lookup_nodes<T: serde::de::DeserializeOwned>(
    src: &str,
) -> Result<impl Iterator<Item = Result<T>>> {
    let path = current_dir()?.join(src);
    let iter = fs::read_dir(path)?.into_iter().map(|path| -> Result<T> {
        let s = fs::read_to_string(path?.path())?;
        let parsed = serde_json::from_str(&s)?;
        Ok(parsed)
    });
    Ok(iter)
}

fn lookup_file<T: serde::de::DeserializeOwned>(path: &str, name: &str) -> Result<T> {
    let path = current_dir()?.join(path).join(format!("{name}.json"));
    let s = fs::read_to_string(&path)?;
    let parsed = serde_json::from_str(&s)?;
    Ok(parsed)
}

fn persist_nodes<T: serde::Serialize>(
    nodes: impl Iterator<Item = (String, T)>,
    dest: &str,
) -> Result<()> {
    let path = current_dir()?.join(dest);
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path)?;
    for (name, node) in nodes {
        let path = path.join(format!("{}.json", name));
        eprintln!("writing: {dest}/{}.json", name);
        let s = serde_json::to_string_pretty(&node)?;
        std::fs::write(path, s.to_string().as_bytes())?;
    }
    Ok(())
}

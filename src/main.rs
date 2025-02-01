use clap::{Command, Arg};
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::collections::HashSet;
use url::Url;

// Function to fetch all links from the given URL
fn fetch_links(url: &str) -> Result<Vec<String>, reqwest::Error> {
    let response = get(url)?;
    let body = response.text()?;

    // Parse the HTML document & select "a" tags (links)
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a").unwrap();
    
    // Iterate through each link and extract the "href" attribute
    let mut links = Vec::new();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            links.push(href.to_string());
        }
    }

    Ok(links)
}

// Function to extract unique domains from a list of links
fn get_domains(links: Vec<String>) -> HashSet<String> {
    let mut domains = HashSet::new();

    // Iterate through each link and extract the domain
    for link in links {
        match Url::parse(&link) {
            Ok(parsed_url) => {
                if let Some(host) = parsed_url.host_str() {
                    domains.insert(host.to_string());
                }
            },
            Err(_) => continue,
        }
    }

    domains
}

fn main() {
    let matches = Command::new("pgls")
        .version("0.1")
        .about("A CLI to fetch and list links and domains")
        // Define the "domains" subcommand
        .subcommand(
            Command::new("domains")
                .about("Fetches the domains from the URL")
                .arg(
                    Arg::new("url")
                        .help("URL to fetch links from")
                        .required(true)
                        .index(1),
                ),
        )
        // Define the "links" subcommand
        .subcommand(
            Command::new("links")
                .about("Lists all links")
                .arg(
                    Arg::new("external")
                        .long("external")
                        .action(clap::ArgAction::SetTrue)
                        .help("List only external links"),
                )
                .arg(
                    Arg::new("internal")
                        .long("internal")
                        .action(clap::ArgAction::SetTrue)
                        .help("List only internal links"),
                )
                .arg(
                    Arg::new("url")
                        .help("URL to fetch links from")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    // Handle the "domains" subcommand
    match matches.subcommand() {
        Some(("domains", sub_matches)) => {
            let url = sub_matches.get_one::<String>("url").unwrap();
            match fetch_links(url) {
                Ok(links) => {
                    let domains = get_domains(links);
                    if domains.is_empty() {
                        println!("None");
                    } else {
                        for domain in domains {
                            println!("{}", domain);
                        }
                    }
                }
                Err(e) => println!("Error fetching URL: {}", e),
            }
        }
        // Handle the "links" subcommand
        Some(("links", sub_matches)) => {
            let url = sub_matches.get_one::<String>("url").unwrap();
            match fetch_links(url) {
                Ok(links) => {
                    let base_url = Url::parse(url).unwrap();
                    // Partition the links into internal and external based on the URL
                    let (internal, external): (Vec<String>, Vec<String>) = links.into_iter()
                        .partition(|link| {
                            match Url::parse(link) {
                                Ok(parsed_url) => parsed_url.host_str() == base_url.host_str(),
                                Err(_) => false,
                            }
                        });

                    // Display external links if the flag is set
                    if sub_matches.get_flag("external") {
                        for link in external {
                            println!("{}", link);
                        }
                    } 
                    // Display internal links if the flag is set
                    else if sub_matches.get_flag("internal") {
                        for link in internal {
                            println!("{}", link);
                        }
                    } 
                    // If no flags are set, display all links (internal + external)
                    else {
                        for link in external.iter().chain(internal.iter()) {
                            println!("{}", link);
                        }
                    }
                }
                Err(e) => println!("Error fetching URL: {}", e),
            }
        }
        _ => println!("Invalid command"),
    }
}

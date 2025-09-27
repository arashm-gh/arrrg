use clap::Parser;
use reqwest::blocking::get;
use serde::Deserialize;
use tabled::{Table, Tabled};

// cli parser
#[derive(Debug, Parser)]
struct Cli {
    query: String,
    num: String,
}

// serde torrent json
#[derive(Debug, Deserialize)]
struct Torrent {
    #[serde(rename = "id")]
    _id: String,
    name: String,
    size: String,
    seeders: String,
    leechers: String,
    info_hash: String,
}

// table display
#[derive(Debug, Tabled)]
struct TorrentRow {
    magnet: String,
    seeds: String,
    size: String,
    leeches: String,
}

fn main() {
    let args = Cli::parse();
    let query = args.query;
    // number argument parser
    let n: u32 = match parse_int(args.num) {
        Err(e) => {
            eprintln!("Error: {}", e);
            println!("Using default value: 10");
            10
        }
        Ok(num) => num,
    };

    // output description
    println!("Top {} Results for: {:?}", n, query);

    // progress bar
    let pb = indicatif::ProgressBar::new(n.into());
    let mut torrents_out: Vec<TorrentRow> = vec![];
    let torrents = fetch_torrents(n, query, &pb);
    for t in torrents.iter().take(n as usize) {
        pb.inc(1);
        torrents_out.push(torrent_to_row(t));
    }
    let table = Table::new(torrents_out).to_string();
    println!("{}", table); // create and display table
}

// using the apibay to fetch the top torrents for any field
// the api works with pages so we're gonna be working on a per page basis
fn fetch_torrents(wanted: u32, query: String, pb: &indicatif::ProgressBar) -> Vec<Torrent> {
    let mut page = 0;
    let mut result: Vec<Torrent> = vec![];

    while result.len() < wanted as usize {
        let url = format!("https://apibay.org/q.php?q={}&page={}", query, page);
        let resp: Vec<Torrent> = get(&url)
            .expect("Error: Unable to reach APIBAY.ORG!")
            .json()
            .expect("Error: Unable to parse to JSON!");

        if resp.is_empty() {
            break;
        }
        pb.inc(resp.len() as u64);
        result.extend(resp);
        page += 1;
    }

    pb.set_length(result.len() as u64);
    result
}

// integer parser
fn parse_int(input_string: String) -> Result<u32, std::num::ParseIntError> {
    let result: Result<u32, std::num::ParseIntError> = input_string.parse();
    result
}

// formatting to magnet links
fn gen_magnet_link(info_hash: &str, name: &str) -> String {
    format!("magnet:?xt=urn:btih:{}&dn={}", info_hash, name)
}

fn torrent_to_row(t: &Torrent) -> TorrentRow {
    TorrentRow {
        seeds: t.seeders.clone(),
        size: t.size.clone(),
        leeches: t.leechers.clone(),
        magnet: gen_magnet_link(&t.info_hash, t.name.as_str()),
    }
}

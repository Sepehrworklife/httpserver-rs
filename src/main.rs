use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};

use clap::{Arg, Command};
use iron::{Iron, Request, Response, headers, status};

fn main() {
    let matches = Command::new("httpserver-rs")
        .version("0.0.1")
        .about("Simple http server written with Rust!")
        .arg(
            Arg::new("directory")
                .help("Directory to serve.")
                .value_parser(|s: &str| -> Result<String, String> {
                    match fs::metadata(s) {
                        Ok(meta) if meta.is_dir() => Ok(s.to_string()),
                        Ok(_) => Err("Not a directory".into()),
                        Err(e) => Err(format!("Error accessing path: {}", e)),
                    }
                })
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .help("Port number")
                .value_parser(clap::value_parser!(i16).range(3000..))
                .default_value("7890"),
        )
        .get_matches();

    let root: PathBuf = matches
        .get_one::<String>("directory")
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap());
    let port: &i16 = matches.get_one::<i16>("port").unwrap();

    let addr = format!("0.0.0.0:{}", port);

    println!("||==========================||");
    println!("||======= HTTP SERVER ======||");
    println!("||========== RUST ==========||");
    println!("||==========================||");
    println!("||== Serving directory: {:?}", root);
    println!("||== Server running on: http://{}", addr);
    println!("||==========================||");
    println!("\r\n");

    Iron::new(move |req: &mut Request| {
        println!("Requested url: {:?}", req.url.path());
        let mut path = root.clone();
        for part in req.url.path() {
            path.push(part);
        }
        match File::open(&path) {
            Ok(f) => {
                // TODO: Add more data to table
                let metadata = f.metadata().unwrap();
                let path_prefix = req
                    .url
                    .path()
                    .into_iter()
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<&str>>();

                let mut files: Vec<String> = Vec::new();
                for entry in fs::read_dir(&path).unwrap() {
                    let entry = entry.unwrap();
                    // TODO: Add more data to table
                    let entry_meta = entry.metadata().unwrap();
                    let file_name = entry.file_name().into_string().unwrap();
                    let mut link = path_prefix.clone();
                    link.push(&file_name);
                    let link = link.join("/");
                    files.push(format!(
                        "<tr><td><a href=\"{}\">{}</a></td></tr>",
                        link, file_name
                    ));
                }
                let html = format!(
                    "<html><body><h1>HTTPServer with Rust!</h1><table border=\"1\">{}</table><p>Author: <a target=\"_blank\" href=\"https://github.com/Sepehrworklife/\">Sepehr Alimohammadi</a></p></body></html>",
                    files.join("\n")
                );
                let mut response = Response::with((status::Ok, html));
                response.headers.set(headers::ContentType::html());
                Ok(response)
            }
            Err(e) => Ok(Response::with((status::NotFound, e.to_string()))),
        }
    })
    .http(addr)
    .unwrap();
}

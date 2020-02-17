use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::File;

#[derive(Serialize, Deserialize)]
struct MyObj {
    matches: Vec<(String, usize)>,
}

#[derive(Serialize, Deserialize)]
struct JsonError {
    description: String,
}

#[derive(Deserialize)]
pub struct SearchParameters {
    motifs: Vec<String>,
}

async fn index(req: web::HttpRequest) -> Result<HttpResponse> {
    if let Ok(parsed) = serde_urlencoded::from_str(req.query_string()) {
        let params: HashMap<String, String> = parsed;
        let motifs: Vec<&str> = params.get("query").unwrap().split(",").collect();

        let mut res = Vec::new();

        let f = File::open("/home/jakob/University/BIOL 396/dm6/dm6.fa").unwrap();
        let (genome, exceptions, chromosomes) = mermer_rs::read_fasta(&f).unwrap();
        let tables = mermer_rs::make_tables(&motifs);
        let repetitions = 1;
        for i in 0..repetitions {
            // TODO: Implement chromosome restrictions.
            let matches = mermer_rs::search(&tables, &genome, 0, genome.len() - 1);
            for (mask, position, depth) in matches {
                let filtered = mermer_rs::identify_matches(mask, position, depth, &genome, &motifs);
                for (motif, position) in filtered {
                    res.push((motif, position));
                }
            }
        }

        Ok(HttpResponse::Ok().json(MyObj { matches: res }))
    } else {
        Ok(HttpResponse::BadRequest().json(JsonError {
            description: String::from("Invalid query string."),
        }))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().route(r"/api/v1/search", web::get().to(index)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

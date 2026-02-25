use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

//static WORDS: Lazy<Mutex<Vec<WordEntry>>> = Lazy::new(|| {
    //let mut rdr = csv::Reader::from_path("unigram_freq.csv").expect("Cannot open CSV");
    //let mut words = Vec::new();

    //for result in rdr.deserialize() {
        //let record: WordEntry = result.expect("Invalid CSV row");
        //words.push(record);
    //}

    //Mutex::new(words)
//});

static WORDS: Lazy<Mutex<Vec<WordEntry>>> = Lazy::new(|| {
    let mut rdr = csv::Reader::from_path("unigram_freq.csv")
        .expect("Cannot open CSV");

    let mut words = Vec::new();

    for result in rdr.records() {
        let record = result.expect("Invalid CSV row");

        let word = record.get(0).unwrap().to_string();
        let frequency: u32 = record.get(1).unwrap().parse().unwrap_or(0);

        if word.len() == 5 {
            words.push(WordEntry { word, frequency });
        }
    }

    Mutex::new(words)
});
#[derive(Debug, Deserialize, Clone)]
struct WordEntry {
    word: String,
    frequency: u32,
}

#[derive(Debug, Deserialize)]
struct GuessInput {
    guess: String,
    pattern: String,
}

#[derive(Debug, Serialize)]
struct Suggestions {
    suggestions: Vec<String>,
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("index.html"))
}

async fn suggest(input: web::Json<Vec<GuessInput>>) -> impl Responder {
    let words = WORDS.lock().unwrap();
    let mut candidates = words.clone();

    for guess in input.iter() {
        candidates = filter_candidates(&candidates, guess);
    }

    // Sort by highest frequency first
    candidates.sort_by(|a, b| b.frequency.cmp(&a.frequency));

    // Convert to list of words only
    let suggestions: Vec<String> =
        candidates.iter().map(|w| w.word.clone()).collect();

    HttpResponse::Ok().json(Suggestions { suggestions })
}

fn filter_candidates(words: &[WordEntry], guess: &GuessInput) -> Vec<WordEntry> {
    words.iter()
        .cloned()
        .filter(|entry| matches_pattern(&entry.word, &guess.guess, &guess.pattern))
        .collect()
}

fn matches_pattern(candidate: &str, guess: &str, pattern: &str) -> bool {
    let candidate_chars: Vec<char> = candidate.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();

    for i in 0..5 {
        match pattern_chars[i] {
            'g' => {
                if candidate_chars[i] != guess_chars[i] {
                    return false;
                }
            }
            'y' => {
                if candidate_chars[i] == guess_chars[i]
                    || !candidate.contains(guess_chars[i])
                {
                    return false;
                }
            }
            'b' => {
                if candidate.contains(guess_chars[i]) {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Wordle solver running at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/suggest", web::post().to(suggest))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

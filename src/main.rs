use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;
use std::env;

#[derive(Deserialize, Debug)]
struct Question {
    _question_id: u32,
    _title: String,
    // title_slug: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run <-q_or_a> <question_number_or_title>");
        println!("-q_or_a: '-q' for question link, '-a' for answer link");
        return Ok(());
    }

    let q_or_a = &args[1];
    let input = args[2].clone();

    if q_or_a != "-q" && q_or_a != "-a" {
        println!(
            "Invalid argument: {}. Use '-q' for question link or '-a' for answer link.",
            q_or_a
        );
        return Ok(());
    }

    let api_url = "https://leetcode.com/api/problems/algorithms/";

    let response = reqwest::get(api_url).await?;
    let json: Value = response.json().await?;

    let questions = json["stat_status_pairs"]
        .as_array()
        .expect("Failed to parse questions");

    let mut found = false;

    for q in questions {
        let question = q["stat"].as_object().unwrap();
        let title_slug = question["question__title_slug"].as_str().unwrap();
        let question_id = question["question_id"].as_u64().unwrap();

        if title_slug == input || question_id.to_string() == input {
            let question_url = format!("https://leetcode.com/problems/{}/", title_slug);
            let answer_url = format!("https://leetcode.com/problems/{}/solution/", title_slug);

            if q_or_a == "-q" {
                println!("Question URL: {}", question_url);
            } else {
                println!("Answer URL: {}", answer_url);
            }

            found = true;
            break;
        }
    }

    if !found {
        println!("Question not found. Please enter a valid question number or title.");
    }

    Ok(())
}

use anyhow::Context;
use itertools::Itertools;
use rayon::collections;
use reqwest::Client;

fn decode(c: char) -> char {
    // TODO: make it a constnat
    let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    return chars[c as usize - 33];
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
struct HistoryRow {
    uuid: String,
    request: String,
    response: String,
    createdAt: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
struct Score {
    problem_name: String,
    score: i64,
    id: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();
    let mut all_history: Vec<HistoryRow> = vec![];

    for page in 1..=100 {
        let url = format!("https://boundvariable.space/team/history?page={}", page);
        let res = client
            .get(url)
            .header("Authorization", icfpc2024::get_bearer_async().await?)
            .send()
            .await?;

        let body = res.text().await?;
        let history: Vec<HistoryRow> = serde_json::from_str(&body)?;

        // Display history
        for row in &history {
            println!(
                "--------------------------------------------------------------------------------"
            );
            println!("UUID: {}", row.uuid);
            println!("Request: {}", row.request);
            println!("Response: {}", row.response);
            println!("Created at: {}", row.createdAt);
            all_history.push(row.clone());
        }
        if history.len() < 100 {
            break;
        }
    }

    let mut scores: Vec<Score> = vec![];
    // https://boundvariable.space/team/history/97a028a2-de54-473a-9eef-3e6adc6eeb51/request
    // Fetch history details into /history of the repository.
    for row in &all_history {
        println!("Processing: {}", row.createdAt);

        // Save the request to a file in the repository.
        // 2024-06-28T13:40:50Z into 20240628-134050.
        let id = row
            .createdAt
            .replace("-", "")
            .replace(":", "")
            .replace("T", "-")
            .replace("Z", "");
        let id = id + "-" + &row.uuid;

        let filename = format!("history/{}/request.txt", id);
        std::fs::create_dir_all(format!("history/{}", id))
            .with_context(|| format!("Failed to create directory history/{}", id))?;
        // If the file already exists, skip.
        if !std::path::Path::new(&filename).exists() {
            let url = format!(
                "https://boundvariable.space/team/history/{}/request",
                row.uuid
            );
            let res = client
                .get(url)
                .header("Authorization", icfpc2024::get_bearer_async().await?)
                .send()
                .await?;

            let body = res.text().await?;
            std::fs::write(&filename, body)
                .with_context(|| format!("Failed to write to {}", filename))?;
        }

        let filename = format!("history/{}/response.txt", id);
        // If the file already exists, skip.
        if !std::path::Path::new(&filename).exists() {
            let url = format!(
                "https://boundvariable.space/team/history/{}/response",
                row.uuid
            );
            let res = client
                .get(url)
                .header("Authorization", icfpc2024::get_bearer_async().await?)
                .send()
                .await?;

            let body = res.text().await?;
            std::fs::write(&filename, body)
                .with_context(|| format!("Failed to write to {}", filename))?;
        }

        let filename = format!("history/{}/response_decoded.txt", id);
        // If the file already exists, skip.
        if !std::path::Path::new(&filename).exists() {
            let source = format!("history/{}/response.txt", id);
            let body = std::fs::read_to_string(&source)
                .with_context(|| format!("Failed to read from {}", source))?;
            if &body[0..1] == "S" {
                let decoded_text = body.chars().skip(1).map(decode).collect::<String>();
                std::fs::write(&filename, decoded_text)
                    .with_context(|| format!("Failed to write to {}", filename))?;
            }
        }

        if std::path::Path::new(&filename).exists() {
            let decoded_text = std::fs::read_to_string(&filename)
                .with_context(|| format!("Failed to read from {}", filename))?;
            // e.g., "Correct, you solved [problem_name] with a score of [score]!"
            // Parse the response and extract the score.
            let parts: Vec<&str> = decoded_text
                .split("\n")
                .next()
                .unwrap()
                .split(" ")
                .collect();
            if parts[0] == "Correct," && parts[1] == "you" && parts[2] == "solved" {
                let problem_name = parts[3].trim_end_matches('!');
                let score = if parts.len() > 8 {
                    parts[8].trim_end_matches('!').parse::<i64>()?
                } else {
                    1
                };
                eprint!("Problem: {}, Score: {}\n", problem_name, score);
                scores.push(Score {
                    problem_name: problem_name.to_string(),
                    score: score,
                    id: id.to_string(),
                });
            }
        }
    }

    // Find best scores.
    scores.sort_by(|a, b| {
        b.problem_name
            .cmp(&a.problem_name)
            .then(b.score.cmp(&a.score).reverse())
    });

    for (problem_name, group) in &scores.into_iter().group_by(|s| s.problem_name.clone()) {
        let mut best_score = std::i64::MAX;
        let mut best_id = "".to_string();
        for score in group {
            if score.score < best_score {
                best_score = score.score;
                best_id = score.id.clone();
            }
        }
        eprintln!(
            "Problem: {}, Best Score: {}, ID: {}",
            problem_name, best_score, best_id
        );
        // Copy history/[id] to best/problem_name
        let source = format!("history/{}", best_id);
        let dest = format!("best/{}", problem_name);
        if std::path::Path::new(&dest).exists() {
            std::fs::remove_dir_all(&dest)
                .with_context(|| format!("Failed to remove directory {}", dest))?;
        }
        std::fs::create_dir_all(&dest)
            .with_context(|| format!("Failed to create directory {}", dest))?;
        let options = fs_extra::dir::CopyOptions {
            overwrite: true,
            content_only: true,
            ..Default::default()
        };
        fs_extra::dir::copy(&source, &dest, &options)
            .with_context(|| format!("Failed to copy from {} to {}", source, dest))?;
    }

    Ok(())
}

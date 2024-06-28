use anyhow::Context;
use reqwest::Client;

fn decode(c: char) -> char {
    // TODO: make it a constnat
    let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    return chars[c as usize - 33];
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct HistoryRow {
    uuid: String,
    request: String,
    response: String,
    createdAt: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://boundvariable.space/team/history?page=1";
    let client = Client::new();
    let res = client
        .get(url)
        .header("Authorization", icfpc2024::get_bearer()?)
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
    }

    // https://boundvariable.space/team/history/97a028a2-de54-473a-9eef-3e6adc6eeb51/request
    // Fetch history details into /history of the repository.
    for row in &history {
        println!("Processing: {}", row.createdAt);

        // Save the request to a file in the repository.
        // 2024-06-28T13:40:50Z into 20240628-134050.
        let id = row
            .createdAt
            .replace("-", "")
            .replace(":", "")
            .replace("T", "-")
            .replace("Z", "");

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
                .header("Authorization", icfpc2024::get_bearer()?)
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
    }

    Ok(())
}

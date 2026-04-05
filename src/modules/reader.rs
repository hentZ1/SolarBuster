use crate::prelude::*;

pub async fn read(path: String, tx: mpsc::Sender<String>) -> Result<()> {
    let file = File::open(&path)
        .await
        .with_context(|| format!("could not open the wordlist: {}", path))?;

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.unwrap() {
        let word = line.trim().to_string();
        if word.is_empty() {
            continue;
        }
        if tx.send(word).await.is_err() {
            break;
        }
    }
    Ok(())
}

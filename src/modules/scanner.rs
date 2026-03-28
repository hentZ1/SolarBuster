use crate::modules::output::print_result;
use crate::prelude::*;

pub async fn dir_scanner(
    client: Client,
    url: String,
    word: String,
    noise: Option<u64>,
    pb: Arc<ProgressBar>,
) {
    let full_url = format!("{}/{}", url.trim_end_matches('/'), word);

    let mut attempts = 0;
    let max_retries = 3;

    while attempts < max_retries {
        match client.get(&full_url).send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() || status.is_redirection() {
                    if let Ok(bytes) = resp.bytes().await {
                        let len = bytes.len() as u64;
                        let is_noise = noise.is_some_and(|n| n > 0 && len == n);
                        if !is_noise {
                            print_result(status, &full_url);
                        }
                    } else {
                        print_result(status, &full_url);
                    }
                }
                break;
            }
            Err(_) => {
                attempts += 1;
                if attempts >= max_retries {
                    break;
                }
            }
        }
    }
    pb.inc(1);
}

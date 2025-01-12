use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Instant,
};
use tokio::sync::{mpsc, RwLock};

use crate::utils::{calculate_combinations, increment, verify_signature};

pub async fn crack_secret(
    header: &jsonwebtoken::Header,
    payload: &serde_json::Value,
    message: &str,
    signature: &[u8],
    alphabet: &str,
    prefix: &str,
    suffix: &str,
    maxlen: usize,
) -> Result<(), String> {
    let alphabet = alphabet.as_bytes();
    let message_bytes = message.as_bytes();

    println!("Parsed JWT:");
    println!("- Algorithm: {:?}", header.alg);
    println!("- Payload: {}", payload);
    println!("- Signature (hex): {}", hex::encode(signature));
    println!();

    let combinations = calculate_combinations(alphabet, maxlen);
    println!("There are {} combinations to attempt", combinations);
    println!("Cracking JWT secret...");

    let start_time = Instant::now();
    let (tx, mut rx) = mpsc::channel(100);
    let found_secret = Arc::new(RwLock::new(None));
    let stop_flag = Arc::new(AtomicBool::new(false));
    let mut attempts: u64 = 0;

    for secret_len in 1..=maxlen {
        let alphabet = alphabet.to_vec();
        let prefix = prefix.to_string();
        let suffix = suffix.to_string();
        let tx = tx.clone();
        let found_secret = Arc::clone(&found_secret);
        let stop_flag = Arc::clone(&stop_flag);

        tokio::spawn(async move {
            generate_secrets(
                secret_len,
                &alphabet,
                &prefix,
                &suffix,
                tx,
                found_secret,
                stop_flag,
            )
            .await;
        });
    }

    while let Some(secret) = rx.recv().await {
        attempts += 1;
        println!("secret: {}", secret);
        if attempts % 100000 == 0 {
            let elapsed = start_time.elapsed().as_secs_f64();
            println!(
                "Attempts: {}, Rate: {:.2} attempts/sec",
                attempts,
                attempts as f64 / elapsed
            );
        }

        if verify_signature(message_bytes, signature, &secret, &header.alg) {
            let mut found_secret = found_secret.write().await;
            if found_secret.is_none() {
                *found_secret = Some(secret.clone());
                stop_flag.store(true, Ordering::Relaxed);
                println!("Found secret: {}", secret);
                break;
            }
        }
    }

    let duration = start_time.elapsed();
    println!("Time elapsed: {:?}", duration);
    Ok(())
}

async fn generate_secrets(
    length: usize,
    alphabet: &[u8],
    prefix: &str,
    suffix: &str,
    tx: mpsc::Sender<String>,
    found_secret: Arc<RwLock<Option<String>>>,
    stop_flag: Arc<AtomicBool>,
) {
    let mut current = vec![0; length];
    let alphabet_len = alphabet.len();

    while !stop_flag.load(Ordering::Relaxed) {
        let secret: String = current.iter().map(|&i| alphabet[i] as char).collect();
        let full_secret = format!("{}{}{}", prefix, secret, suffix);

        if tx.send(full_secret).await.is_err() {
            break;
        }

        if !increment(&mut current, alphabet_len) {
            break;
        }
    }
}

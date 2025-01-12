use jwt_cracker_rs::{crack_secret, decode_jwt, parse_args};

#[tokio::main]
async fn main() {
    let args = parse_args();
    match decode_jwt(&args.token) {
        Ok((header, payload, message, signature)) => {
            if let Err(err) = crack_secret(
                &header,
                &payload,
                &message,
                &signature,
                &args.alphabet,
                &args.prefix,
                &args.suffix,
                args.maxlen,
            )
            .await
            {
                eprintln!("Failed to crack JWT secret: {}", err);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

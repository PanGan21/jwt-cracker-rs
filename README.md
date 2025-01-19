# wait-for-rs

A high-performance Rust-based command-line tool for brute-forcing secrets of JSON Web Tokens (JWTs). It supports HMAC algorithms and showcases potential vulnerabilities in JWTs with weak secrets. Use this tool responsibly for educational or security research purposes.

## Features

- **Efficient Brute Force**: Utilizes asynchronous concurrency for optimal secret generation and verification.
- **Supports HMAC Algorithms**: Compatible with HS256, HS384, and HS512 JWTs.
- **Real-Time Feedback**: Displays progress metrics, including attempts and rate of processing.
- **Highly Configurable**: Customize the alphabet, prefix, suffix, and maximum secret length.

### Installation

Clone the repository and build the project:

```bash
git clone https://github.com/PanGan21/jwt-cracker-rs.git
cd jwt-cracker-rs && cargo install --path .
```

### Usage

#### Command-Line Tool

To start the server:

```bash
jwt-cracker-rs \
  --token "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.XbPfbIHMI6arZ3Y922BhjWgQzWXcXNrz0ogtVhfEd2o" \
  --alphabet "abcdefghijklmnopqrstuwxyz" \
  --maxlen 6
```

#### Example Output

```
Found secret: secret
Time elapsed: 1292.404555038s
```

## Disclaimer

This tool is intended for educational purposes and security research only. The misuse of `jwt-cracker-rs` to attack systems without permission is illegal and unethical. Always obtain explicit authorization before testing the security of any system.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

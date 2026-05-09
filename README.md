# SolarBuster

Async web directory enumerator written in Rust. Built as a learning project focused on async concurrency and cross-platform delivery.

## How It Works

SolarBuster reads a wordlist line by line, sends concurrent GET requests to `<url>/<word>`, and prints any path that returns a 2xx or 3xx status. Before scanning, it sends one request to a fake path to measure the response size of 404/wildcard pages — any result with that exact byte length is silently skipped.

Concurrency is controlled by a `tokio::sync::Semaphore`. Words flow from the file into a bounded `mpsc` channel (capacity: 1000) and are consumed by spawned tasks up to the configured worker limit.

```
reader (async file) ──► mpsc channel ──► semaphore-gated tasks ──► HTTP GET ──► print if valid
                                                                  └── noise filter (body size)
```

Failed requests retry up to 3 times before being dropped.

## Install

**Pre-built binary** — download from [Releases](https://github.com/hentZ1/SolarBuster/releases):

| Platform | File |
|----------|------|
| Linux x86_64 | `SolarBuster-linux-x86_64` |
| Windows x86_64 | `SolarBuster-windows-x86_64.exe` |
| macOS Intel | `SolarBuster-macos-x86_64` |
| macOS Apple Silicon | `SolarBuster-macos-arm64` |

**Build from source** (requires Rust stable, edition 2024):

```bash
git clone https://github.com/hentZ1/SolarBuster.git
cd SolarBuster
cargo build --release
```

## Usage

```
SolarBuster -u <URL> -w <WORDLIST> [-c <CONCURRENCY>]
```

| Flag | Long | Default | Description |
|------|------|---------|-------------|
| `-u` | `--url` | required | Target base URL |
| `-w` | `--wordlist` | required | Path to wordlist file |
| `-c` | `--concurrency` | `50` | Max concurrent workers |

```bash
./SolarBuster -u https://example.com -w wordlist.txt
./SolarBuster -u https://example.com -w wordlist.txt -c 100
```

## Security Assessment

| Feature | Status | Detail |
|---------|--------|--------|
| HTTPS / TLS | ✅ | `reqwest` with native TLS; certificates validated |
| Wildcard detection | ✅ Basic | Compares response body byte size to a noise baseline |
| Retry on error | ✅ | Up to 3 retries per word on network failure |
| Timeout | ✅ Fixed | 5 s connect + 5 s read (not configurable via CLI) |
| User-Agent | ⚠ | Hard-coded as `SolarBuster` — trivially detected/blocked |
| Rate limiting | ❌ | No delay between requests |
| Proxy support | ❌ | Not implemented |
| Auth / headers | ❌ | No cookie, token, or custom header injection |
| Redirect depth | ❌ | Redirects are printed but not followed |

## Dependencies

| Crate | Role |
|-------|------|
| `tokio` | Async runtime |
| `reqwest` | HTTP client |
| `clap` | CLI argument parsing |
| `indicatif` | Progress spinner |
| `colored` | Terminal colors |
| `figlet-rs` | ASCII banner |
| `anyhow` | Error handling |

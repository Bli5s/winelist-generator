# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build and run locally
cargo run

# Production build
cargo build --release

# Docker
docker build -t winelist-generator .
docker run -p 8000:8000 -e CELLARTRACKER_USR=... -e CELLARTRACKER_PW=... winelist-generator
```

No test suite is configured.

## Required Environment Variables

- `CELLARTRACKER_USR` — CellarTracker username
- `CELLARTRACKER_PW` — CellarTracker password

## Architecture

Single-file Rust application (`src/main.rs`, ~117 lines) that generates a PDF wine list from a CellarTracker inventory. When a request hits `GET /`, the application:

1. Fetches wine inventory as CSV from the CellarTracker API (`xlquery.asp`)
2. Parses and deduplicates into a `HashSet<Wine>` (7 fields per wine)
3. Renders a LaTeX document organized by wine variety
4. Converts LaTeX → PDF via the `pandoc` binary
5. Returns the PDF at `static/winelist.pdf`

**Key detail:** Vintage `"1001"` maps to `"N.V."` (Non-Vintage).

## Runtime Dependencies

The application shells out to `pandoc` and requires a full TeX Live installation. The Dockerfile installs both (`texlive-full`, `pandoc`) on Ubuntu. On macOS, install these manually before running locally.

## Stack

- **Rocket 0.5** — async HTTP server (listens on `0.0.0.0:8000`)
- **Reqwest** — HTTP client for CellarTracker API calls
- **csv crate** — CSV parsing
- **latex + pandoc crates** — LaTeX document generation and PDF conversion
- **failure** — error types

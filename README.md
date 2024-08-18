# md-blog-rs

Code for my personal website. Simple, markdown-based.

## What it does

- Serves markdown content
- Uses a basic HTML template
- Background grid shared between everyone for fun
- My portfolio/blog

## Run it

```bash
cargo run --release [HOST] [PORT]
```

Default: 127.0.0.1:8080

## Files

- `main.rs`: Server
- `index.html`: Page template
- `markdown/`: Content

Built with Rust and Actix.
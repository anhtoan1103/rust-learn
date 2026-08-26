# PowerShell script to run cargo-watch (install if missing)
if (-not (Get-Command cargo-watch -ErrorAction SilentlyContinue)) {
    Write-Host "cargo-watch not found — installing via 'cargo install cargo-watch'..."
    cargo install cargo-watch
}

Write-Host "Starting auto-reload: cargo watch -x run"
cargo watch -x run

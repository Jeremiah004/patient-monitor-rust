# vitalswatch

A small Rust tool that reads a stream of simulated vital signs (heart rate and body temperature) and flags anomalies in real time using a rolling z-score detector.

Status: early work in progress. The data source is simulated for now.

## How it works

- `sensor.rs` generates simulated readings with injected spikes.
- `detector.rs` keeps a rolling window of recent values and flags readings that sit far from the recent mean. Flagged values are kept out of the window so one spike does not corrupt the baseline.
- `main.rs` is the command-line interface. Alerts can be written to CSV or JSON.

## Usage

```
cargo run -- --ticks 60 --delay-ms 20 --csv alerts.csv --json alerts.json
cargo run -- --help
```

Options: `--ticks`, `--delay-ms`, `--threshold` (z-score, default 3.0), `--window` (default 10), `--csv`, `--json`.

## Tests

```
cargo test
```

## Limitations

- Readings are simulated, not from a real device.
- The z-score method is simple. It assumes roughly stable data and is not a clinical alert system.

## Roadmap

- Read real sensor data (ESP32 or similar)
- Per-metric configuration
- Live web dashboard

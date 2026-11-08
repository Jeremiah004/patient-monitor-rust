use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::File,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};
use vitalswatch::{alert::Alert, detector::Detector, sensor::simulate};

#[derive(Parser)]
#[command(name = "vitalswatch", about = "Real-time vital-sign anomaly detector")]
struct Args {
    /// Number of readings to simulate
    #[arg(long, default_value_t = 100)]
    ticks: u32,

    /// Delay between readings in milliseconds
    #[arg(long, default_value_t = 200)]
    delay_ms: u64,

    /// Z-score above which a reading is flagged
    #[arg(long, default_value_t = 3.0)]
    threshold: f64,

    /// Rolling window size (number of recent readings)
    #[arg(long, default_value_t = 10)]
    window: usize,

    /// Write alerts to this CSV file
    #[arg(long)]
    csv: Option<PathBuf>,

    /// Write alerts to this JSON file
    #[arg(long)]
    json: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut hr_detector = Detector::new(args.window, args.threshold, 2.0);
    let mut temp_detector = Detector::new(args.window, args.threshold, 0.1);
    let mut alerts: Vec<Alert> = Vec::new();

    for tick in 0..args.ticks {
        let r = simulate(tick);
        let mut flagged = false;

        if let Some(z) = hr_detector.check(r.heart_rate) {
            alerts.push(Alert {
                tick,
                metric: "heart_rate".into(),
                value: r.heart_rate,
                z_score: z,
            });
            flagged = true;
        }
        if let Some(z) = temp_detector.check(r.temp_c) {
            alerts.push(Alert {
                tick,
                metric: "temp_c".into(),
                value: r.temp_c,
                z_score: z,
            });
            flagged = true;
        }

        let flag = if flagged { "  <-- ALERT" } else { "" };
        println!(
            "[{:03}] HR: {:.1} bpm | Temp: {:.2} °C{flag}",
            r.tick, r.heart_rate, r.temp_c
        );
        thread::sleep(Duration::from_millis(args.delay_ms));
    }

    if let Some(path) = &args.csv {
        write_csv(path, &alerts)?;
    }
    if let Some(path) = &args.json {
        write_json(path, &alerts)?;
    }

    println!("\nDone. {} alerts raised.", alerts.len());
    Ok(())
}

fn write_csv(path: &Path, alerts: &[Alert]) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)
        .with_context(|| format!("cannot create {}", path.display()))?;
    for alert in alerts {
        writer.serialize(alert)?;
    }
    writer.flush()?;
    Ok(())
}

fn write_json(path: &Path, alerts: &[Alert]) -> Result<()> {
    let file = File::create(path).with_context(|| format!("cannot create {}", path.display()))?;
    serde_json::to_writer_pretty(file, alerts)?;
    Ok(())
}

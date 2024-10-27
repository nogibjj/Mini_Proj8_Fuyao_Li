use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

pub fn benchmark_file_io() -> std::io::Result<f64> {
    let start = Instant::now();

    // Use BufWriter to improve write performance by buffering output
    let file = File::create("data_sample_rust.txt")?;
    let mut writer = BufWriter::new(file);

    for i in 0..500000 {
        writeln!(writer, "Sample data line {}", i)?;
    }

    // Ensure all data is flushed to disk
    writer.flush()?;

    let duration = start.elapsed().as_secs_f64();
    println!("Optimized Rust File Write Time: {:.2} seconds", duration);

    Ok(duration)
}

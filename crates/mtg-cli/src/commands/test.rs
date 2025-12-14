//! Test commands

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

/// Run recognition tests on image catalog
pub async fn recognition(catalog: &str, format: &str) -> anyhow::Result<()> {
    println!("{}", "Running recognition tests...".cyan());
    println!("Catalog: {}", catalog);
    println!();

    // TODO: Load test images and run recognition
    let test_images = vec![
        ("lightning-bolt.jpg", "Lightning Bolt", true),
        ("counterspell.jpg", "Counterspell", true),
        ("blurry-card.jpg", "Unknown", false),
    ];

    let pb = ProgressBar::new(test_images.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut passed = 0;
    let mut failed = 0;

    for (image, expected, should_pass) in &test_images {
        pb.inc(1);
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // TODO: Actually run recognition
        if *should_pass {
            passed += 1;
        } else {
            failed += 1;
        }

        let _ = (image, expected); // Suppress unused warnings
    }

    pb.finish_and_clear();

    match format {
        "json" => {
            println!(
                "{}",
                serde_json::json!({
                    "total": test_images.len(),
                    "passed": passed,
                    "failed": failed,
                    "accuracy": (passed as f64 / test_images.len() as f64) * 100.0
                })
            );
        }
        "csv" => {
            println!("image,expected,actual,correct");
            for (image, expected, correct) in &test_images {
                println!("{},{},{},{}", image, expected, expected, correct);
            }
        }
        _ => {
            println!("{}", "Test Results".bold());
            println!();
            println!("  {} {}", "Total:".bold(), test_images.len());
            println!("  {} {}", "Passed:".bold(), format!("{}", passed).green());
            println!("  {} {}", "Failed:".bold(), format!("{}", failed).red());
            println!(
                "  {} {:.1}%",
                "Accuracy:".bold(),
                (passed as f64 / test_images.len() as f64) * 100.0
            );
            println!();

            if failed > 0 {
                println!("{}", "Failed tests:".red().bold());
                for (image, expected, correct) in &test_images {
                    if !correct {
                        println!("  {} {} (expected: {})", "✗".red(), image, expected);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Run performance benchmarks
pub async fn benchmark(iterations: usize) -> anyhow::Result<()> {
    println!("{}", "Running performance benchmarks...".cyan());
    println!("Iterations: {}", iterations);
    println!();

    let pb = ProgressBar::new(iterations as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut times: Vec<u128> = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let start = std::time::Instant::now();

        // TODO: Actually run recognition
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        times.push(start.elapsed().as_millis());
        pb.inc(1);
    }

    pb.finish_and_clear();

    times.sort();
    let p50 = times[times.len() / 2];
    let p95 = times[(times.len() as f64 * 0.95) as usize];
    let p99 = times[(times.len() as f64 * 0.99) as usize];
    let avg: u128 = times.iter().sum::<u128>() / times.len() as u128;

    println!("{}", "Benchmark Results".bold());
    println!();
    println!("  {} {}ms", "Average:".bold(), avg);
    println!("  {} {}ms", "P50:".bold(), p50);
    println!("  {} {}ms", "P95:".bold(), p95);
    println!("  {} {}ms", "P99:".bold(), p99);
    println!("  {} {}ms", "Min:".bold(), times.first().unwrap());
    println!("  {} {}ms", "Max:".bold(), times.last().unwrap());

    Ok(())
}

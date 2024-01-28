use clap::{Parser, ValueHint};
use chrono::{Datelike, Local, NaiveDate, Weekday};
use eyre::{WrapErr, Result};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The date for which to calculate the work week, in YYYY-MM-DD format.
    #[clap(value_parser, default_value_t = default_date(), value_hint = ValueHint::Other, help = "The date for which to calculate the work week, in YYYY-MM-DD format.")]
    date: String,
}

// Function to provide a default date string for the current day.
fn default_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Parse the provided date or return an error if it's not valid.
    let date = NaiveDate::parse_from_str(&args.date, "%Y-%m-%d")
        .wrap_err_with(|| format!("Could not parse the date: {}", args.date))?;

    let work_week = calculate_work_week(date)
        .wrap_err("Failed to calculate the work week")?;

    println!("WW{}", work_week);

    Ok(())
}

fn calculate_work_week(date: NaiveDate) -> Result<u32> {
    let year = date.year();
    let first_jan = NaiveDate::from_ymd_opt(year, 1, 1)
        .ok_or_else(|| eyre::eyre!("Invalid start date"))?;
    let first_sunday = if first_jan.weekday() == Weekday::Sun {
        first_jan
    } else {
        first_jan
            + chrono::Duration::days(7 - first_jan.weekday().num_days_from_sunday() as i64)
    };
    let work_week = ((date.ordinal() - first_sunday.ordinal()) / 7) + 1;
    Ok(work_week)
}

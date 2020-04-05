//! `clock` - Cross-platform NTP client and system time control
mod clock;
mod ntp;

use chrono::{format::ParseResult, DateTime, Duration as ChronoDuration, FixedOffset, Utc};
use clap::{arg_enum, value_t, App, AppSettings, Arg, SubCommand};

use clock::Clock;
use ntp::check_time;

arg_enum! {
    #[derive(Debug)]
    enum TimeFormat {
        RFC2822,
        RFC3339,
        Timestamp,
    }
}

fn parse_timestamp(s: &str) -> ParseResult<DateTime<FixedOffset>> {
    DateTime::parse_from_str(s, "%s")
}

/// Check for OS errors from attempts to *set* the time. This should be called
/// immediately after calls to `Clock::set`.
///
/// NOTE: This really ought to be done with `Result` return values but
/// the authors went down this route to avoid spending more time on the
/// `Clock::set` methods. I also suspect that it's just another way to
/// introduce the reader to other error handling methods (e.g. using
/// the OS' error reporting) available in Rust.
fn check_time_set_error() {
    let maybe_error = std::io::Error::last_os_error();
    let os_error_code = &maybe_error.raw_os_error();
    match os_error_code {
        Some(0) => (),
        None => (),
        _ => {
            eprintln!("Unable to set the time: {:?}", maybe_error);
            std::process::exit(1);
        }
    }
}

fn main() {
    let default_format = format!("{:?}", TimeFormat::RFC3339);
    let app = App::new("clock")
        .version("0.1.3")
        .about("Gets and sets the time.")
        .after_help("Note: UNIX timestamps are parsed as whole seconds since 1st January 1970 0:00:00 UTC. For more accuracy, use another format.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("std")
                .short("s")
                .long("use-standard")
                .help("Standard time format for parsing/output (case insensitive).")
                .takes_value(true)
                .possible_values(&TimeFormat::variants())
                .case_insensitive(true)
                .default_value(&default_format))
        .subcommand(
            SubCommand::with_name("get")
            .about("Get the current system time"))
        .subcommand(
            SubCommand::with_name("set")
            .about("Set the current system time")
            .arg(Arg::with_name("datetime").required(true)))
        .subcommand(
            SubCommand::with_name("check-ntp")
            .about("Adjust the current system time using NTP")
            .arg(Arg::with_name("query")
                .short("q")
                .help("Query only - don't set the clock."))
        );

    let args = app.get_matches();

    // Default value set in clap app -> safe to unwrap
    let std = value_t!(args, "std", TimeFormat).unwrap();

    match args.subcommand() {
        ("get", _) => {
            let now = Clock::get();
            match std {
                TimeFormat::Timestamp => println!("{}", now.timestamp()),
                TimeFormat::RFC2822 => println!("{}", now.to_rfc2822()),
                TimeFormat::RFC3339 => println!("{}", now.to_rfc3339()),
            }
        }
        ("set", Some(sub_args)) => {
            // Required by clap arg -> safe to unwrap
            let t_ = sub_args.value_of("datetime").unwrap();
            let parser = match std {
                TimeFormat::RFC2822 => DateTime::parse_from_rfc2822,
                TimeFormat::RFC3339 => DateTime::parse_from_rfc3339,
                TimeFormat::Timestamp => parse_timestamp,
            };

            let err_msg = format!("Unable to parse {} according to {}", t_, std);
            let t = parser(t_).expect(&err_msg);

            Clock::set(t);
            check_time_set_error();
        }
        ("check-ntp", Some(sub_args)) => {
            // Perform as much setup as possible before checking and/or setting
            // the clock to avoid introducing noise to the time.
            let should_set = !sub_args.is_present("query");
            let offset = check_time().unwrap() as isize;
            println!("Avg offset: {}", offset);
            // TODO: Extract this logic and test
            let adjust_ms = offset.signum() * offset.abs().min(200) / 5;
            let adjust_ms = ChronoDuration::milliseconds(adjust_ms as i64);
            println!("Adjustment ms: {}", adjust_ms);
            let now: DateTime<Utc> = Utc::now() + adjust_ms;

            if should_set {
                Clock::set(now);
                check_time_set_error();
                println!("Set time: {}", now);
            } else {
                println!("Would set time: {}", now);
            }
        }
        _ => unreachable!(),
    }
}

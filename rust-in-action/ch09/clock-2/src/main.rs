//! `clock` - Cross-platform system time control
//!
//! Demonstration of retrieving system time and using OS APIs to *set* the
//! system time on both Windows and `libc` OS's.
use chrono::{DateTime, Local, TimeZone};
use clap::{App, Arg};
use std::mem::zeroed;

struct Clock; // zero-sized type -> no memory usage -> compile time only

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    /// Set the system time - OS errors are *not* handled here and should be
    /// checked for using `std::io::Error::last_os_error`.
    ///
    /// Uses `libc:settimeofday` to set the system time to `t`.
    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) {
        use libc::{
            settimeofday,
            suseconds_t, // fractional component of current second
            time_t,      // seconds that have elapsed since the Epoch
            timeval,
            timezone,
        };

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            // settimeofday generates an errow with non-null timezone values
            // -> some kind of error/historic issue?
            let mock_tz: *const timezone = std::ptr::null();
            // Note: This should stay as the *last* OS call in this function
            // to allow error checking by the caller.
            settimeofday(&u as *const timeval, mock_tz);
        }
    }

    /// Set the system time - OS errors are *not* handled here and should be
    /// checked for using `std::io::Error::last_os_error`.
    ///
    /// Uses `kernel32::SetSystemTime` to set the system time to `t`.
    #[cfg(windows)]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) {
        use kernel32::{GetSystemTime, SetSystemTime};
        use winapi::{SYSTEMTIME, WORD};

        let t = t.with_timezone(&Local);

        let mut systime: SYSTEMTIME = unsafe { zeroed() };

        // Day number mapping from https://docs.microsoft.com/en-gb/windows/win32/api/minwinbase/ns-minwinbase-systemtime#members
        let dow = match t.weekday() {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 0,
        };

        // chrono represents leap seconds by adding an extra second *within*
        // the nanosecond field. Since the windows API requires milliseconds,
        // the additional nanosecond needs to be taken into account during
        // conversion.
        // https://docs.rs/chrono/0.4.11/chrono/naive/struct.NaiveTime.html#leap-second-handling
        let mut ns = t.nanosecond();
        let mut leap_second = 0;
        let is_leap = ns > 1000000000;

        if is_leap {
            ns -= 1000000000;
            leap_second += 1;
        }

        systime.wYear = t.year() as WORD;
        systime.wMonth = t.month() as WORD;
        systime.wDayOfWeek = dow as WORD;
        systime.wDay = t.day() as WORD;
        systime.wHour = t.hour() as WORD;
        systime.wMinute = t.minute() as WORD;
        // TODO: Is this correct? SYSTEMTIME docs do not mention leap seconds
        // does this imply that seconds field must remain 0..60?
        // https://msdn.microsoft.com/en-us/library/windows/desktop/ms724950(v=vs.85).aspx
        systime.wSecond = (leap_second + t.second()) as WORD;
        systime.wMilliseconds = (ns / 1_000_000) as WORD;

        let systime_ptr = &systime as *const SYSTEMTIME;

        unsafe {
            // Note: This should stay as the *last* OS call in this function
            // to allow error checking by the caller.
            SetSystemTime(systime_ptr);
        }
    }
}

fn main() {
    let app = App::new("clock")
        .version("0.1.2")
        .about("Gets and sets the time.")
        .arg(
            Arg::with_name("action")
                .takes_value(true)
                .possible_values(&["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::with_name("std")
                .short("s")
                .long("use-standard")
                .takes_value(true)
                .possible_values(&["rfc2822", "rfc3339", "timestamp"])
                .default_value("rfc3339"),
        )
        .arg(
            Arg::with_name("datetime")
                .help("When <action> is 'set', apply <datetime>. Otherwise, ignore.")
                // Force user to provide if using the "set" action
                .required_if("action", "set"),
        );

    let args = app.get_matches();

    // Default values set in clap app -> safe to unwrap
    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();

    if action == "set" {
        // Required by clap arg -> safe to unwrap
        let t_ = args.value_of("datetime").unwrap();
        let parser = match std {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!(),
        };

        let err_msg = format!("Unable to parse {} according to {}", t_, std);
        let t = parser(t_).expect(&err_msg);

        // Attempt to change time, failures from calls to platform-specific
        // OS time functions are *not* checked for
        Clock::set(t);

        // Pick up OS time function failure from ^
        //
        // NOTE: This really ought to be done with `Result` return values but
        // the authors went down this route to avoid spending more time on the
        // `Clock::set` methods. I also suspect that it's just another way to
        // introduce the reader to other error handling methods (e.g. using
        // the OS' error reporting) available in Rust.
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
    } else {
        let now = Clock::get();
        match std {
            "timestamp" => println!("{}", now.timestamp()),
            "rfc2822" => println!("{}", now.to_rfc2822()),
            "rfc3339" => println!("{}", now.to_rfc3339()),
            _ => unreachable!(), // Clap app restricts possible values
        }
    }
}

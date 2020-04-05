//! System clock utility
use chrono::{DateTime, Local, TimeZone};
use std::mem::zeroed;

#[cfg(not(windows))]
use libc::{
    settimeofday,
    suseconds_t, // fractional component of current second
    time_t,      // seconds that have elapsed since the Epoch
    timeval,
    timezone,
};

#[cfg(windows)]
use {
    chrono::{Datelike, Timelike, Weekday},
    kernel32::SetSystemTime,
    winapi::{SYSTEMTIME, WORD},
};

#[cfg(not(windows))]
fn to_os_time<Tz: TimeZone>(t: DateTime<Tz>) -> timeval {
    let mut u: timeval = unsafe { zeroed() };

    u.tv_sec = t.timestamp() as time_t;
    u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;
    u
}

#[cfg(windows)]
fn to_os_time<Tz: TimeZone>(t: DateTime<Tz>) -> SYSTEMTIME {
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
    let is_leap = ns > 1_000_000_000;

    if is_leap {
        ns -= 1_000_000_000;
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
    systime
}

/// Interface to the system clock.
pub struct Clock;

impl Clock {
    /// Return the current system time.
    pub fn get() -> DateTime<Local> {
        Local::now()
    }

    /// Set the system time - OS errors are *not* handled here and should be
    /// checked for using `std::io::Error::last_os_error`.
    ///
    /// Uses `libc:settimeofday` to set the system time to `t`.
    #[cfg(not(windows))]
    pub fn set<Tz: TimeZone>(t: DateTime<Tz>) {
        let t = t.with_timezone(&Local);
        let u = to_os_time(t);
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
    pub fn set<Tz: TimeZone>(t: DateTime<Tz>) {
        let t = t.with_timezone(&Local);
        let systime = to_os_time(t);
        let systime_ptr = &systime as *const SYSTEMTIME;

        unsafe {
            // Note: This should stay as the *last* OS call in this function
            // to allow error checking by the caller.
            SetSystemTime(systime_ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, ParseResult, Utc};

    #[test]
    fn test_to_os_time() -> ParseResult<()> {
        let t = DateTime::<Utc>::from_utc(
            NaiveDate::from_ymd(1998, 7, 6).and_hms_micro(8, 30, 30, 1000),
            Utc,
        );

        // Neither timeval or SYSTIME implement PartialEq -> manual assertions
        // required for each member

        #[cfg(not(windows))]
        {
            let u = to_os_time(t);
            assert_eq!(u.tv_sec, 899713830);
            assert_eq!(u.tv_usec, 1000);
        }

        #[cfg(windows)]
        {
            let systime = to_os_time(t);

            assert_eq!(systime.wYear, 1998);
            assert_eq!(systime.wMonth, 7);
            assert_eq!(systime.wDayOfWeek, 1);
            assert_eq!(systime.wDay, 6);
            assert_eq!(systime.wHour, 8);
            assert_eq!(systime.wMinute, 30);
            assert_eq!(systime.wSecond, 30);
            assert_eq!(systime.wMilliseconds, 1);
        }
        Ok(())
    }

    #[test]
    fn test_to_os_time_leap_second() -> ParseResult<()> {
        // chrono represents leap seconds by *adding* an additional second in
        // to the *nanosecond* field. Thus, this DateTime represents
        // 1998-07-06T08:30:00.5+00:00 + 1 leap second
        let t = DateTime::<Utc>::from_utc(
            NaiveDate::from_ymd(1998, 7, 6).and_hms_micro(8, 30, 30, 1_500_000),
            Utc,
        );

        // Neither timeval or SYSTIME implement PartialEq -> manual assertions
        // required for each member

        #[cfg(not(windows))]
        {
            let u = to_os_time(t);
            assert_eq!(u.tv_sec, 899713830);
            assert_eq!(u.tv_usec, 1_500_000);
        }

        #[cfg(windows)]
        {
            let systime = to_os_time(t);

            assert_eq!(systime.wYear, 1998);
            assert_eq!(systime.wMonth, 7);
            assert_eq!(systime.wDayOfWeek, 1);
            assert_eq!(systime.wDay, 6);
            assert_eq!(systime.wHour, 8);
            assert_eq!(systime.wMinute, 30);
            assert_eq!(systime.wSecond, 31);
            assert_eq!(systime.wMilliseconds, 500);
        }
        Ok(())
    }
}

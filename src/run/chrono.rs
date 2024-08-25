use std::{
    fmt::{self, Display},
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq)]
enum ChronometerState {
    Stopped,
    Running,
    Paused,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum ChronometerFormat {
    HHMMSSX,
    HHMMSS,
    HHMM,
    MMSSX,
    MMSS,
}

impl ChronometerFormat {
    pub fn text(&self) -> &str {
        match self {
            ChronometerFormat::HHMMSSX => "HH:MM:SS.cs",
            ChronometerFormat::HHMMSS => "HH:MM:SS",
            ChronometerFormat::HHMM => "HH:MM",
            ChronometerFormat::MMSSX => "MM:SS.cs",
            ChronometerFormat::MMSS => "MM:SS",
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chronometer {
    start_time: Option<Instant>,
    elapsed: Option<Duration>,
    state: ChronometerState,
    format: ChronometerFormat,
}

impl Chronometer {
    pub fn new(format: ChronometerFormat) -> Self {
        Self {
            start_time: None,
            elapsed: None,
            state: ChronometerState::Stopped,
            format,
        }
    }

    pub fn set_format(&mut self, format: &ChronometerFormat) {
        self.format = format.clone();
    }

    pub fn load_chrono(&mut self, elapsed: Duration, format: &ChronometerFormat) {
        self.start_time = None;
        self.elapsed = Some(elapsed);
        self.state = ChronometerState::Paused;
        self.format = format.clone();
    }

    pub fn clear_elapsed(&mut self) {
        self.start_time = Some(Instant::now());
        self.elapsed = Some(Duration::default());
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.state = ChronometerState::Running;
    }

    pub fn pause(&mut self) {
        if self.state == ChronometerState::Running {
            self.elapsed =
                Some(self.start_time.unwrap().elapsed() + self.elapsed.unwrap_or_default());
            self.state = ChronometerState::Paused;
        }
    }

    pub fn reset(&mut self) {
        if self.state != ChronometerState::Stopped {
            self.start_time = None;
            self.elapsed = None;
            self.state = ChronometerState::Stopped;
        }
    }

    pub fn get_time(&self) -> Duration {
        let mut time = self.elapsed.unwrap_or_default();
        if self.state == ChronometerState::Running {
            time += self.start_time.unwrap().elapsed();
        }
        time
    }
}

impl Display for Chronometer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            duration_chrono_format(self.elapsed.unwrap_or_default(), &self.format)
        )
    }
}

pub fn duration_chrono_format(duration: Duration, format: &ChronometerFormat) -> String {
    let total_millis = duration.as_millis();
    let total_secs = duration.as_secs();

    match format {
        ChronometerFormat::HHMMSSX => {
            let xs = (total_millis % 1000) / 10;
            let ss = total_secs % 60;
            let mm = (total_secs / 60) % 60;
            let hh = total_secs / 3600;
            format!("{:}:{:02}:{:02}.{:02}", hh, mm, ss, xs)
        }
        ChronometerFormat::HHMMSS => {
            let ss = total_secs % 60;
            let mm = (total_secs / 60) % 60;
            let hh = total_secs / 3600;
            format!("{:}:{:02}:{:02}", hh, mm, ss)
        }
        ChronometerFormat::HHMM => {
            let mm = (total_secs / 60) % 60;
            let hh = total_secs / 3600;
            format!("{:}:{:02}", hh, mm)
        }
        ChronometerFormat::MMSSX => {
            let xs = (total_millis % 1000) / 10;
            let ss = total_secs % 60;
            let mm = total_secs / 60;
            format!("{:02}:{:02}.{:02}", mm, ss, xs)
        }
        ChronometerFormat::MMSS => {
            let ss = total_secs % 60;
            let mm = total_secs / 60;
            format!("{:02}:{:02}", mm, ss)
        }
    }
}

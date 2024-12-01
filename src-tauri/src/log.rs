// src/log.rs

use std::time::{Duration, SystemTime};

pub struct Log {
    pub story_id: String,
    pub start_time: Option<SystemTime>,
    pub end_time: Option<SystemTime>,
}

impl Log {
    pub fn new(story_id: String) -> Self {
        Log {
            story_id,
            start_time: None,
            end_time: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(SystemTime::now());
    }

    pub fn stop(&mut self) {
        self.end_time = Some(SystemTime::now());
    }

    pub fn duration(&self) -> Option<Duration> {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => end.duration_since(start).ok(),
            _ => None,
        }
    }
}

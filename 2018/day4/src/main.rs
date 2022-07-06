#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Write};
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    let mut events = Vec::new();
    for line in s.lines() {
        if line.is_empty() {
            continue;
        }
        events.push(Event::from_str(line)?);
    }
    events.sort_by(|a, b| a.datetime.cmp(&b.datetime));
    for event in events.iter() {
        println!("{:?}", event);
    }
    let mut guard_event: HashMap<i32, Vec<&Event>> = HashMap::new();
    let mut cur_guard_id = None;
    for event in &events {
        if let EventType::Shift { guard_id } = event.event_type {
            cur_guard_id = Some(guard_id);
        }
        match cur_guard_id {
            None => return Err("No guard id for event".into()),
            Some(id) => {
                guard_event.entry(id).or_default().push(event);
            }
        }
    }

    part_1(&guard_event)?;
    part_2(&guard_event)?;

    Ok(())
}

fn part_1(guard_event: &HashMap<i32, Vec<&Event>>) -> Result<()> {
    let mut guard_sleep_time: HashMap<i32, i32> = HashMap::new();
    for (id, events) in guard_event {
        let mut fall_into_sleep = None;
        for event in events {
            match event.event_type {
                EventType::Sleep => {
                    if fall_into_sleep == None {
                        fall_into_sleep = Some(event.datetime.total_minutes());
                    }
                }
                EventType::Awake => match fall_into_sleep {
                    Some(time) => {
                        let minutes = event.datetime.total_minutes() - 1 - time;
                        guard_sleep_time.entry(*id).or_insert(minutes);

                        let curr_sleep_time = guard_sleep_time.get_mut(id).unwrap();

                        if *curr_sleep_time < minutes {
                            *curr_sleep_time = minutes;
                        }

                        fall_into_sleep = None;
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }
    }
    let (id, time) = guard_sleep_time.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();

    writeln!(io::stdout(), "{:?}", id*time)?;
    Ok(())
}

fn part_2(guard_event: &HashMap<i32, Vec<&Event>>) -> Result<()> {
    Ok(())
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug)]
struct Event {
    datetime: DateTime,
    event_type: EventType,
}

#[derive(Debug)]
enum EventType {
    Shift { guard_id: i32 },
    Sleep,
    Awake,
}

impl DateTime{
    fn total_minutes(&self) -> i32{
        (self.day * 24 * 60 + self.hour * 60 + self.minute) as i32
    }
}

impl FromStr for Event {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                            \[
                            (?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})
                            \s+
                            (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})
                            \]
                            \s+
                            (Guard\ \#(?P<id>[0-9]+)\ begins\ shift|(?P<event>.+))
                            ",
            )
            .unwrap();
        };

        let caps = match RE.captures(s) {
            Some(caps) => caps,
            None => Err("Unrecognized event")?
        };

        let datetime = DateTime {
            year: caps["year"].parse()?,
            month: caps["month"].parse()?,
            day: caps["day"].parse()?,
            hour: caps["hour"].parse()?,
            minute: caps["minute"].parse()?,
        };

        let event_type = if let Some(t) = caps.name("id") {
            EventType::Shift {
                guard_id: t.as_str().parse()?,
            }
        } else if &caps["event"] == "falls asleep" {
            EventType::Sleep
        } else if &caps["event"] == "wakes up" {
            EventType::Awake
        } else {
            return Err("Unknown event type".into());
        };

        Ok(Event {
            datetime,
            event_type,
        })
    }
}

#![cfg(test)]

use std::collections::HashMap;

use itertools::Itertools;
use lazy_regex::regex_captures;

#[derive(Debug)]
enum Action {
    BeginShift(u16),
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
struct Event {
    when: String,
    minute: u8,
    action: Action,
}

// Parse lines of the form:
//
// [1518-10-05 00:10] falls asleep
// [1518-07-22 23:53] Guard #1949 begins shift
// [1518-07-06 00:55] wakes up
fn parse_line(line: &str) -> Event {
    if let Some((_, when, minute, action, guard)) = regex_captures!(
        r#"^\[(\d+-\d+-\d+ \d+:(\d+))\] (falls asleep|wakes up|Guard #(\d+) begins shift)$"#,
        line
    ) {
        Event {
            when: when.to_string(),
            minute: minute.parse().unwrap(),
            action: if action == "falls asleep" {
                Action::FallAsleep
            } else if action == "wakes up" {
                Action::WakeUp
            } else {
                Action::BeginShift(guard.parse().unwrap())
            },
        }
    } else {
        panic!("bad input line: {}", line.escape_debug())
    }
}

fn compute(input: &str) -> (usize, usize) {
    let mut events: Vec<Event> = input.lines().map(parse_line).collect();
    events.sort_unstable_by(|a, b| a.when.cmp(&b.when));
    assert!(events.len() > 0);
    assert!(matches!(events[0].action, Action::BeginShift(_)));
    let tagged_events: Vec<(u16, Event)> = tag_events(events);
    let sleep_windows: Vec<(u16, u8, u8)> = sleep_windows(tagged_events);

    // Map Dwarf ID to a histogram of sleep minutes.
    let sleep_histograms: HashMap<u16, Vec<u32>> = sleep_histograms(sleep_windows);

    // Strategy 1: Find the guard that has the most minutes asleep. What minute
    // does that guard spend asleep the most?
    let answer1 = compute_strategy(&sleep_histograms, sleep_minutes_sum);

    // Strategy 2: Of all guards, which guard is most frequently asleep on the
    // same minute? What minute does that guard spend asleep the most?
    let answer2 = compute_strategy(&sleep_histograms, sleep_minutes_max);

    (answer1, answer2)
}

fn compute_strategy(
    sleep_histograms: &HashMap<u16, Vec<u32>>,
    max_strategy: fn(&[u32]) -> u32,
) -> usize {
    let answer1 = {
        let most_sleep: Option<(&u16, &Vec<u32>)> = {
            sleep_histograms
                .iter()
                .max_by_key(|(_, histogram)| max_strategy(histogram))
        };
        let sleepiest_minute: Option<(usize, &u32)> = most_sleep
            .unwrap()
            .1
            .iter()
            .enumerate()
            .max_by_key(|(_i, c)| *c);

        let (id, _) = most_sleep.unwrap();
        let (minute, _) = sleepiest_minute.unwrap();
        dbg!(&id);
        dbg!(&minute);
        (*id as usize) * minute
    };
    answer1
}

fn sleep_minutes_sum(histogram: &[u32]) -> u32 {
    histogram.iter().sum::<u32>()
}

fn sleep_minutes_max(histogram: &[u32]) -> u32 {
    histogram.iter().max().copied().unwrap_or(0)
}

fn sleep_histograms(sleep_windows: Vec<(u16, u8, u8)>) -> HashMap<u16, Vec<u32>> {
    let sleep_histograms: HashMap<u16, Vec<u32>> = {
        sleep_windows
            .iter()
            .fold(HashMap::new(), |mut histograms, (id, start, end)| {
                let minutes: &mut Vec<u32> = histograms.entry(*id).or_default();
                let start: usize = (*start).into();
                let end: usize = (*end).into();
                if minutes.len() < end {
                    minutes.resize(end, 0);
                }
                for minute in &mut minutes[start..end] {
                    *minute += 1;
                }
                histograms
            })
    };
    sleep_histograms
}

fn sleep_windows(tagged_events: Vec<(u16, Event)>) -> Vec<(u16, u8, u8)> {
    let sleep_windows: Vec<(u16, u8, u8)> = {
        let mut vec = Vec::new();
        for (a, b) in tagged_events.iter().tuple_windows() {
            match (a, b) {
                (
                    (
                        id_a,
                        Event {
                            when: _,
                            minute: fall_asleep_minute,
                            action: Action::FallAsleep,
                        },
                    ),
                    (
                        _id_b,
                        Event {
                            when: _,
                            minute: wake_up_minute,
                            action: Action::WakeUp,
                        },
                    ),
                ) => {
                    vec.push((*id_a, *fall_asleep_minute, *wake_up_minute));
                }
                _ => {}
            }
        }
        vec
    };
    sleep_windows
}

fn tag_events(events: Vec<Event>) -> Vec<(u16, Event)> {
    let tagged_events: Vec<(u16, Event)> = events
        .into_iter()
        .scan(0, |id, event| {
            if let Action::BeginShift(event_id) = event.action {
                *id = event_id;
            }
            Some((*id, event))
        })
        .filter(|(_, event)| !matches!(event.action, Action::BeginShift(_)))
        .collect();
    tagged_events
}

mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test() {
        let (answer1, answer2) = compute(INPUT);
        assert_eq!(answer1, 21083);
        assert_eq!(answer2, 53024);
    }
}

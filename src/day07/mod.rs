use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

// Return two tasks (prerequisite, dependent) where the prerequisite
// task must be completed before the dependent.
fn parse_line(line: &str) -> (u8, u8) {
    let parsed: Vec<u8> = line
        .split_whitespace()
        .filter_map(|s| {
            if s.len() == 1 && s.as_bytes()[0].is_ascii_uppercase() {
                Some(s.as_bytes()[0] - b'A')
            } else {
                None
            }
        })
        .collect();
    assert_eq!(2, parsed.len());
    (parsed[0], parsed[1])
}

// Return a map of dependent tasks [0..26) mapped to a bitmask of prerequisite
// tasks.
fn parse(input: &str) -> BTreeMap<u8, u32> {
    let mut task_prerequisites = BTreeMap::new();

    // Populate all keys [0..26) with zero.
    for task in 0_u8..26_u8 {
        task_prerequisites.entry(task).or_default();
    }

    for line in input.lines() {
        let (prerequisite, dependent) = parse_line(line);
        let prerequisite = 1_u32 << prerequisite;
        task_prerequisites
            .entry(dependent)
            .and_modify(|p| *p |= prerequisite);
    }
    task_prerequisites
}

pub fn compute_part_one(input: &str) -> String {
    let mut task_prerequisites = parse(input);
    dbg!(&task_prerequisites);
    let mut completed = 0_u32;
    let mut result = String::new();
    while !task_prerequisites.is_empty() {
        dbg!(&task_prerequisites);
        let task = *task_prerequisites
            .iter()
            .find_map(|(task, prerequisites)| {
                if completed & *prerequisites == *prerequisites {
                    Some(task)
                } else {
                    None
                }
            })
            .unwrap();
        dbg!(task);
        completed |= 1_u32 << task;
        task_prerequisites.remove(&task);
        result.push((b'A' + task) as char);
    }
    result
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Task {
    done_at_sec: u32,
    task_index: u8,
}

pub fn compute_part_two(input: &str) -> u32 {
    let mut task_prerequisites = parse(input);
    let mut completed = 0_u32;
    let mut working: BinaryHeap<Reverse<Task>> = BinaryHeap::new();
    let mut now_sec = 0_u32;
    let mut result = String::new();
    while !task_prerequisites.is_empty() {
        let finished_tasks = {
            let mut finished_tasks = Vec::new();
            if let Some(Reverse(task)) = working.pop() {
                let done_at_sec = task.done_at_sec;
                finished_tasks.push(task);
                while let Some(Reverse(task)) = working.peek() {
                    if task.done_at_sec != done_at_sec {
                        break;
                    }
                    finished_tasks.push(*task);
                    working.pop();
                }
            }
            finished_tasks
        };

        for task in finished_tasks {
            now_sec = task.done_at_sec;
            completed |= 1_u32 << task.task_index;
            task_prerequisites.remove(&task.task_index);
            result.push((b'A' + task.task_index) as char);
        }

        for (task_index, prerequisites) in task_prerequisites.iter_mut() {
            // Stop if there are too many pending tasks.
            if working.len() == 5 {
                break;
            }

            // If this task can be completed enqueue the worker.
            if completed & *prerequisites == *prerequisites {
                working.push(Reverse(Task {
                    // The problem statement requires tasks take 60 seconds
                    // plus one second for each letter.  E.g. 'A' contributes
                    // one additonal second.  We convert 'A' to zero, add +1
                    // to the sum of 60 the task index.
                    done_at_sec: now_sec + 60 + (*task_index as u32) + 1,
                    task_index: *task_index,
                }));
                // Mark this task with impossible prerequisites so it is not
                // chosen again.
                *prerequisites = u32::MAX;
            }
        }
    }
    now_sec
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(compute_part_one(INPUT), "BITRAQVSGUWKXYHMZPOCDLJNFE");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(compute_part_two(INPUT), 869);
    }
}

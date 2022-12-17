use regex::Regex;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Valve {
    flow_rate: usize,
    tunnels: Vec<String>,
}

pub fn puzzle_a(input: &str) -> usize {
    let mut map = HashMap::new();

    let pattern =
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)")
            .unwrap();

    for line in input.lines() {
        let caps = pattern.captures(line).unwrap();

        let k = caps.get(1).unwrap().as_str();
        let v = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(',')
            .map(|e| e.trim())
            .map(|e| e.to_string())
            .collect::<Vec<_>>();
        let r = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let valve = Valve {
            flow_rate: r,
            tunnels: v,
        };

        map.insert(k, valve);
    }

    let opened: HashSet<String> = HashSet::new();
    let mut cached: HashMap<(u8, String, usize), usize> = HashMap::new();
    let ans = dfs(1, "AA", 0, 0, &map, &opened, &mut cached);
    ans.unwrap()
}

pub fn puzzle_b(input: &str) -> usize {
    let mut map = HashMap::new();

    let pattern =
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)")
            .unwrap();

    for line in input.lines() {
        let caps = pattern.captures(line).unwrap();

        let k = caps.get(1).unwrap().as_str();
        let v = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(',')
            .map(|e| e.trim())
            .map(|e| e.to_string())
            .collect::<Vec<_>>();
        let r = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let valve = Valve {
            flow_rate: r,
            tunnels: v,
        };

        map.insert(k, valve);
    }

    let opened: HashSet<String> = HashSet::new();
    let mut cached: HashMap<(u8, String, String, usize), usize> = HashMap::new();
    let ans = dfs2(1, "AA", "AA", 0, 0, &map, &opened, &mut cached);
    ans.unwrap()
}

fn dfs(
    minutes: u8,
    current: &str,
    flow_rate: usize,
    current_rate: usize,
    loop_up: &HashMap<&str, Valve>,
    opened: &HashSet<String>,
    cache: &mut HashMap<(u8, String, usize), usize>,
) -> Option<usize> {
    if minutes > 30 {
        return Some(current_rate);
    }

    let key = (minutes, current.to_string(), flow_rate);
    match cache.get(&key) {
        Some(val) if val >= &current_rate => {
            return None;
        }
        _ => {}
    }

    cache.insert(key, current_rate);

    let loop_up_value = loop_up.get(&current).unwrap();

    let best = if loop_up_value.flow_rate > 0 && !opened.contains(current) {
        let mut new_opened = opened.clone();
        new_opened.insert(current.to_string());

        let new_score = current_rate + flow_rate;
        let new_flow_rate = flow_rate + loop_up_value.flow_rate;

        dfs(
            minutes + 1,
            current,
            new_flow_rate,
            new_score,
            loop_up,
            &new_opened,
            cache,
        )
    } else {
        None
    };

    let bests = loop_up_value
        .tunnels
        .iter()
        .filter_map(|e| {
            dfs(
                minutes + 1,
                e,
                flow_rate,
                current_rate + flow_rate,
                loop_up,
                opened,
                cache,
            )
        })
        .max();

    bests.max(best)
}

fn dfs2(
    minutes: u8,
    my_current: &str,
    elephant_current: &str,
    flow_rate: usize,
    current_rate: usize,
    loop_up: &HashMap<&str, Valve>,
    opened: &HashSet<String>,
    cache: &mut HashMap<(u8, String, String, usize), usize>,
) -> Option<usize> {
    if minutes > 26 {
        return Some(current_rate);
    }

    let cache_key = (
        minutes,
        my_current.to_string(),
        elephant_current.to_string(),
        flow_rate,
    );
    if let Some(cached_value) = cache.get(&cache_key) {
        if *cached_value >= current_rate {
            return None;
        }
    }
    cache.insert(cache_key, current_rate);

    let (my_flow_rate, my_tunnels) = {
        let valve = loop_up.get(my_current).unwrap();
        (valve.flow_rate, valve.tunnels.to_vec())
    };
    let (elephant_flow_rate, elephant_tunnels) = {
        let valve = loop_up.get(elephant_current).unwrap();
        (valve.flow_rate, valve.tunnels.to_vec())
    };

    let can_open_my_valve = my_flow_rate > 0 && !opened.contains(my_current);
    let can_open_elephant_valve = elephant_flow_rate > 0 && !opened.contains(elephant_current);
    let mut results = Vec::new();

    if can_open_my_valve {
        // open my valve, elephant moves
        let mut new_open_valves = opened.iter().cloned().collect::<HashSet<_>>();
        new_open_valves.insert(my_current.to_string());

        for new_elephant_location in elephant_tunnels.iter() {
            results.push(dfs2(
                minutes + 1,
                my_current,
                new_elephant_location,
                flow_rate + my_flow_rate,
                current_rate + flow_rate,
                loop_up,
                &new_open_valves,
                cache,
            ));
        }
    }

    if can_open_elephant_valve {
        // open elephant valve, i move
        let mut new_open_valves = opened.iter().cloned().collect::<HashSet<_>>();
        new_open_valves.insert(elephant_current.to_string());

        for new_my_location in my_tunnels.iter() {
            results.push(dfs2(
                minutes + 1,
                new_my_location,
                elephant_current,
                flow_rate + elephant_flow_rate,
                current_rate + flow_rate,
                loop_up,
                &new_open_valves,
                cache,
            ));
        }
    }

    if can_open_elephant_valve && can_open_my_valve && my_current != elephant_current {
        // elephant and i open our valves
        let mut new_open_valves = opened.iter().cloned().collect::<HashSet<_>>();
        new_open_valves.insert(elephant_current.to_string());
        new_open_valves.insert(my_current.to_string());

        results.push(dfs2(
            minutes + 1,
            my_current,
            elephant_current,
            flow_rate + my_flow_rate + elephant_flow_rate,
            current_rate + flow_rate,
            loop_up,
            &new_open_valves,
            cache,
        ));
    }

    // both elephant and i move
    for new_elephant_location in elephant_tunnels.iter() {
        for new_my_location in my_tunnels.iter() {
            results.push(dfs2(
                minutes + 1,
                new_my_location,
                new_elephant_location,
                flow_rate,
                current_rate + flow_rate,
                loop_up,
                opened,
                cache,
            ));
        }
    }

    results.into_iter().flatten().max()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_a_works() {
        let input = include_str!("../input/day16.test.in");
        assert_eq!(puzzle_a(input), 1651);
    }

    #[test]
    fn test_puzzle_b_works() {
        let input = include_str!("../input/day16.test.in");
        assert_eq!(puzzle_b(input), 1707);
    }
}

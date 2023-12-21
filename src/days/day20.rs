use std::{
    collections::{VecDeque, HashMap},
    cell::RefCell, borrow::BorrowMut,
};

use regex::Regex;

// In this problem, we use bools for the pulses: low = false, high = true. 

#[derive(Debug, Clone)]
pub enum ModuleHandler {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Untyped(Option<bool>),
    Broadcaster,
}

impl ModuleHandler {
    pub fn handle(&mut self, pulse: bool, from: String) -> Option<bool> {
        match self {
            Self::FlipFlop(state) => {
                if !pulse {
                    *state = !*state;
                    Some(state.clone())
                } else {
                    None
                }
            },
            Self::Conjunction(state) => {
                state.insert(from, pulse);
                Some(!state.iter().all(|(_, p)| *p ))
            },
            Self::Untyped(state) => { *state = Some(pulse); None },
            Self::Broadcaster => Some(pulse),
        }
    }

    pub fn val(&mut self) -> Option<bool> {
        match self {
            Self::Untyped(val) => { 
                let tmp = val.clone(); 
                *val = None; 
                tmp
            },
            _ => panic!("Unexpected variant for value. "),
        }
    }

    pub fn add_src(&mut self, src: String) {
        match self {
            ModuleHandler::Conjunction(state) => {
                state.insert(src, false);
            },
            _ => {},
        }
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub handler: ModuleHandler,
    pub destinations: Vec<String>,
}

pub fn parse_modules(input: &str) -> HashMap<String, RefCell<Module>> {
    let line_regex = Regex::new("([&|%]*)([a-z]+) -> ([a-z, ]+)").unwrap();
    let mut modules = HashMap::new();

    for line in input.lines() {
        let caps = line_regex.captures(line).unwrap();
        let key = caps[2].to_string();
        let destinations: Vec<String> = caps[3].split(", ").map(str::to_string).collect();

        let handler = match &caps[1] {
            "%" => ModuleHandler::FlipFlop(false),
            "&" => ModuleHandler::Conjunction(HashMap::new()),
            _ => {
                if key == "broadcaster" {
                    ModuleHandler::Broadcaster
                } else {
                    ModuleHandler::Untyped(None)
                }
            }
        };

         // Check that we have the desinations in the modules, else add untyped. 
         for dest in destinations.iter() {
            if !modules.contains_key(dest) {
                modules.insert(dest.clone(), RefCell::new(Module {
                    handler: ModuleHandler::Untyped(None),
                    destinations: vec![],
                }));
            }
         }

        let module = Module {
            handler: handler,
            destinations: destinations,
        };

        modules.insert(key, RefCell::new(module));
    }

    // Do a sweep to link up the dests. 
    for (key, m) in modules.iter() {
        for dest in m.borrow().destinations.iter() {
            modules.get(dest).unwrap().borrow_mut().handler.add_src(key.clone());
        }
    }

    modules
}

#[cfg(test)]
mod tests {

    use std::borrow::Borrow;

    use super::*;
    use num::integer::lcm;

    const INPUT: &str = include_str!("../../data/day20/input.txt");

    #[test]
    fn day20_part1() {
        let modules = parse_modules(INPUT);
        
        let npulse = 1000;
        let mut low_pulse: i64 = 0;
        let mut high_pulse: i64 = 0;
        let mut queue: VecDeque<(String, String, bool)> = VecDeque::new();
        for _ipulse in 0..npulse {
            // Inject the first pulse into the broadcast module. 
            queue.push_back(("".to_string(), "broadcaster".to_string(), false));
            low_pulse += 1;

            while let Some((from, to, input)) = queue.pop_front() {
                let mut m = modules.get(&to).unwrap().borrow_mut();
                if let Some(output) = m.handler.handle(input, from) {
                    for dest in m.destinations.iter() {
                        if output {
                            high_pulse += 1;
                        } else {
                            low_pulse += 1;
                        }
                        queue.push_back((to.clone(), dest.clone(), output))
                    }
                }
            }
        }

        assert_eq!(low_pulse * high_pulse, 929810733);
    }

    #[test]
    fn day20_part2() {
        let modules = parse_modules(INPUT);

        // I've studied my input, and the module that feeds into `rx` is `lg`.
        // So I want to keep track of all modules that feed into that.
        let (output_mod, _) = modules.clone().into_iter().find(|(_, m)| m.clone().borrow().destinations.contains(&"rx".to_string())).unwrap();
        let output_mod = output_mod.clone().to_string();
        let mut cycle_lengths: HashMap<String, i64> = HashMap::new();
        
        let mut queue: VecDeque<(String, String, bool)> = VecDeque::new();
        let mut icycle = 0;

        while cycle_lengths.len() < 4 {
            // Inject the first pulse into the broadcast module. 
            queue.push_back(("".to_string(), "broadcaster".to_string(), false));
            icycle += 1;

            while let Some((from, to, input)) = queue.pop_front() {
                let mut m = modules.get(&to).unwrap().borrow_mut();
                if let Some(output) = m.handler.handle(input, from.clone()) {
                    for dest in m.destinations.iter() {
                        queue.push_back((to.clone(), dest.clone(), output));

                        if dest == &output_mod && output == true {
                            cycle_lengths.insert(from.clone(), icycle);
                        }
                    }
                }
            }
        }

        let min_presses = cycle_lengths.iter().fold(1, |accum, (_, cyc)| lcm(accum, *cyc));
        assert_eq!(min_presses, 231657829136023);
    }
}
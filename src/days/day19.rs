use regex::Regex;

#[derive(Debug, Clone)]
pub struct Part {
    x: i32,
    m: i32,
    a: i32, 
    s: i32,
}

impl Part {
    pub fn from_str(input: &str) -> Part {
        let pat = Regex::new("\\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\\}").unwrap();
        let caps = pat.captures(input).unwrap();
        Part { 
            x: caps[1].parse().unwrap(), 
            m: caps[2].parse().unwrap(), 
            a: caps[3].parse().unwrap(), 
            s: caps[4].parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WorkflowOutcome {
    Accept,
    Reject,
    Workflow(String),
    Rule(String),
}

impl WorkflowOutcome {
    pub fn from_str(input: &str) -> WorkflowOutcome {
        match input {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => if input.contains(":") {
                Self::Rule(input.to_string())
            } else {
                Self::Workflow(input.to_string())
            },
        }
    }
}

pub fn eval_rule(part: &Part, rule: String) -> WorkflowOutcome {
    let eval_pat = regex::Regex::new("([x|m|a|s])([<|>])([0-9]+)").unwrap();

    // Get the command from the string. 
    let (expr, options) = rule.split_once(":").unwrap();
    let caps = eval_pat.captures(&rule).unwrap();
    let val = match &caps[1] {
        "x" => part.x,
        "m" => part.m,
        "a" => part.a,
        "s" => part.s,
        _ => panic!("Invalid part parameter. "),
    };
    let cmp: i32 = caps[3].parse().unwrap();
    let result = match &caps[2] {
        "<" => val < cmp,
        ">" => val > cmp,
        _ => panic!("Unexpected operator. "),
    };

    let (acc, rej) = options.split_once(",").unwrap();
    let outcome = if result {
        WorkflowOutcome::from_str(acc)
    } else {
        WorkflowOutcome::from_str(rej)
    };

    match outcome {
        WorkflowOutcome::Accept | WorkflowOutcome::Reject | WorkflowOutcome::Workflow(_)=> outcome,
        WorkflowOutcome::Rule(rule) => eval_rule(part, rule)
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::*;

    const INPUT: &str = include_str!("../../data/day19/input.txt");

    #[test]
    fn day19_part1() {
        let workflows: HashMap<String, String> = INPUT
            .lines()
            .take_while(|line| *line != "")
            .map(|line| {
                let (name, rules) = line.split_once("{").unwrap();
                (name.to_string(), rules.replace("}", "").to_string())
            })
            .collect();

        let parts: Vec<Part> = INPUT
            .lines()
            .skip_while(|line| *line != "")
            .skip(1)
            .map(|line| Part::from_str(line))
            .collect();
        
        let mut tot = 0;
        for part in parts {
            let mut res = WorkflowOutcome::Workflow(String::from("in"));
            loop {
                match res {
                    WorkflowOutcome::Accept => {
                        tot += part.x + part.m + part.a + part.s;
                        break;
                    },
                    WorkflowOutcome::Reject => break,
                    WorkflowOutcome::Rule(_) => panic!("Should not make it to here. "),
                    WorkflowOutcome::Workflow(wf) => {
                        res = eval_rule(&part, workflows[&wf].clone());
                    }
                }
            }
        }

        assert_eq!(tot, 330820);
    }
}
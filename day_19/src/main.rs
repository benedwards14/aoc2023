use std::{fs::read_to_string, collections::HashMap};

fn parse_input() -> (HashMap<String, Vec<(Option<Condition>, Branch)>>, Vec<Item>)  {
    let mut workflows = HashMap::new();
    let mut items = Vec::new();

    let input = read_to_string("./data.txt").unwrap();
    let mut lines = input.lines();
    let mut row = lines.next().unwrap();
    while !row.is_empty() {
        let (instruction_id, code) = row.split_once("{").unwrap();
        let mut conditions = Vec::new();
        for branch in code.trim_end_matches("}").split(",") {
            let parsed_branch = if let Some((condition, result)) = branch.split_once(":") {
                (Some(Condition::parse(condition)), Branch::parse(result))
            } else {
                (None, Branch::parse(branch))
            };
            conditions.push(parsed_branch)
        }

        workflows.insert(instruction_id.to_owned(), conditions);

        row = lines.next().unwrap();
    }

    for row in lines {
        items.push(Item::parse(row.trim_matches('{').trim_matches('}').split(",").collect()))
    }

    (workflows, items)
}

#[derive(Debug)]
struct Item {
    x: i64,
    m: i64,
    a: i64,
    s: i64 
}

impl Item {
    fn parse(attributes: Vec<&str>) -> Self {
        let mut attribute_map:HashMap<&str, i64> = HashMap::new();
        for a in attributes {
            let (name, value) = a.split_once("=").unwrap();
            attribute_map.insert(name, value.parse().unwrap());
        }

        Self { 
            x: *attribute_map.get("x").unwrap(), 
            m: *attribute_map.get("m").unwrap(), 
            a: *attribute_map.get("a").unwrap(), 
            s: *attribute_map.get("s").unwrap() }
    }

    fn value(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
enum Comparison {
    GreaterThan,
    LessThan
}

impl Comparison {
    fn parse(line: &str) -> Self {
        match line {
            ">" => Self::GreaterThan,
            "<" => Self::LessThan,
            _ => { unreachable!(); }
            
        }
    }

    fn evaluate(&self, item_value: i64, other_value: i64) -> bool {
        match self {
            Self::GreaterThan => item_value > other_value,
            Self::LessThan => item_value < other_value
        }
    }

    fn update(&self, l: &Limit, new_value: i64) -> Limit {
        match self {
            Self::GreaterThan => if new_value > l.lower {
                Limit { upper: l.upper, lower: new_value }
            } else { l.clone() },
            Self::LessThan => if new_value < l.upper {
                Limit { upper: new_value, lower: l.lower }
            } else { l.clone() },
        }
    }

    fn update_not(&self, l: &Limit, new_value: i64) -> Limit {
        match self {
            Self::LessThan => if new_value > l.lower {
                Limit { upper: l.upper, lower: new_value -1 }
            } else { l.clone() },
            Self::GreaterThan => if new_value < l.upper {
                Limit { upper: new_value + 1, lower: l.lower }
            } else { l.clone() },
        }
    }
    
}

#[derive(Debug)]
enum Condition {
    X(Comparison, i64),
    M(Comparison, i64),
    A(Comparison, i64),
    S(Comparison, i64),
}

impl Condition {
    fn parse(line: &str) -> Self {
        let attribute = &line[..1];
        let condition = Comparison::parse(&line[1..2]);
        let value: i64 = line[2..].parse().unwrap();

        match attribute {
            "x" => Self::X(condition, value),
            "m" => Self::M(condition, value),
            "a" => Self::A(condition, value),
            "s" => Self::S(condition, value),
            _ => { unreachable!(); }
        }
    }

    fn evaluate(&self, item: &Item) -> bool {
        match self {
            Self::X(cond, value) => cond.evaluate(item.x, *value),
            Self::M(cond, value) => cond.evaluate(item.m, *value),
            Self::A(cond, value) => cond.evaluate(item.a, *value),
            Self::S(cond, value) => cond.evaluate(item.s, *value),
        }
    }

    fn update(&self, l: &AttributeLimit) -> (AttributeLimit, AttributeLimit) {
        match self {
            Self::X(cond, value) => (
                AttributeLimit { x: cond.update(&l.x, *value), m: l.m.clone(), a: l.a.clone(), s: l.s.clone()}, 
                AttributeLimit { x: cond.update_not(&l.x, *value), m: l.m.clone(), a: l.a.clone(), s: l.s.clone()}
            ),
            Self::M(cond, value) => (
                AttributeLimit { x: l.x.clone(), m: cond.update(&l.m, *value), a: l.a.clone(), s: l.s.clone()}, 
                AttributeLimit { x: l.x.clone(), m: cond.update_not(&l.m, *value), a: l.a.clone(), s: l.s.clone() }
            ),
            Self::A(cond, value) => (
                AttributeLimit { x: l.x.clone(), m: l.m.clone(), a: cond.update(&l.a, *value), s: l.s.clone()}, 
                AttributeLimit { x: l.x.clone(), m: l.m.clone(), a: cond.update_not(&l.a, *value), s: l.s.clone() }
            ),
            Self::S(cond, value) => (
                AttributeLimit { x: l.x.clone(), m: l.m.clone(), a: l.a.clone(), s: cond.update(&l.s, *value)}, 
                AttributeLimit { x: l.x.clone(), m: l.m.clone(), a: l.a.clone(), s: cond.update_not(&l.s, *value)}
            ),
        }
    }
}

#[derive(Debug)]
enum Branch {
    Instruction(String),
    Accept,
    Reject
}

impl Branch {
    fn parse(line: &str) -> Self {
        match line {
            "R" => Self::Reject,
            "A" => Self::Accept,
            a => Self::Instruction(a.to_owned())
        }
    }
}

#[derive(Clone, Debug)]
struct Limit{
    lower: i64,
    upper: i64
}

#[derive(Clone, Debug)]
struct AttributeLimit {
    x: Limit,
    m: Limit,
    a: Limit,
    s: Limit
}

fn evaluate_workflow<'a>(workflow: &'a Vec<(Option<Condition>, Branch)>, item: &Item) -> &'a Branch {
    for (condition, branch) in workflow {
        if let Some(cond) = condition {
            if cond.evaluate(item) {
                return branch;
            }
        } else {
            return branch;
        }
    }

    unreachable!()
}

fn evaluate_item(workflows: &HashMap<String, Vec<(Option<Condition>, Branch)>>, item: &Item) -> bool {
    let mut curr = "in";

    loop {
        let workflow = workflows.get(curr).unwrap();
        let result = evaluate_workflow(workflow, item);
        match result {
            Branch::Accept => { return true; },
            Branch::Reject => { return false; },
            Branch::Instruction(a) => { curr = a; },
        }
    }
}

fn get_combinations_recursive(workflows: &HashMap<String, Vec<(Option<Condition>, Branch)>>, current: &str, mut curr_limit: AttributeLimit) -> Vec<AttributeLimit> {
    let workflow = workflows.get(current).unwrap();
    let mut found_limits = Vec::new();

    for (condition, branch) in workflow {
        let (passed_limits, rejected_limits) = if let Some(cond) = condition {
            cond.update(&curr_limit)
        } else {
            (curr_limit.clone(), curr_limit)
        };

        match branch {
            Branch::Accept => { found_limits.push(passed_limits.clone()) },
            Branch::Reject => {} ,
            Branch::Instruction(i) => {found_limits.extend(get_combinations_recursive(workflows, i, passed_limits.clone())) },
        }

        curr_limit = rejected_limits;
    }

    found_limits
}


fn get_combinations(workflows: &HashMap<String, Vec<(Option<Condition>, Branch)>>) -> Vec<AttributeLimit> {
    get_combinations_recursive(
        workflows, 
        "in", 
        AttributeLimit {
            x: Limit { upper: 4001, lower: 0 },
            m: Limit { upper: 4001, lower: 0 },
            a: Limit { upper: 4001, lower: 0 },
            s: Limit { upper: 4001, lower: 0 }
        }
    )
}

fn main() {
    let (workflows, items) = parse_input();

    let total: i64 = items.iter()
        .filter(|i| evaluate_item(&workflows, i))
        .map(|i| i.value())
        .sum();

    println!("{}", total);

    let combinations = get_combinations(&workflows);
    let combs_total: i64 = combinations.iter()
        .map(|c| (c.x.upper - c.x.lower - 1) * (c.m.upper - c.m.lower - 1) * (c.a.upper - c.a.lower - 1) * (c.s.upper - c.s.lower - 1))
        .sum();

    println!("{}", combs_total);
}

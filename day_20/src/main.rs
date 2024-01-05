use std::{fs::read_to_string, collections::{HashMap, VecDeque}, clone};

fn parse_input() -> HashMap<String, Box<dyn Module>>  {
    let mut modules = HashMap::new();

    let input = read_to_string("./data.txt").unwrap();
    for row in input.lines() {
        let (module_str, destination_str) = row.split_once("->").unwrap();

        let destinations: Vec<String> = destination_str.split(", ").map(|d| d.trim().to_owned()).collect();
        let (name, module): (String, Box<dyn Module>) =  match module_str.trim() {
            "broadcaster" => ("broadcaster".to_owned(), Box::new(Broadcaster { destinations })),
            module if module.starts_with('%') => (module[1..].to_owned(), Box::new (FlipFlop { is_on: false, destinations} )),
            module if module.starts_with('&') => (module[1..].to_owned(), Box::new(Conjuction { inputs: HashMap::new(), destinations })),
            _ => { unreachable!(); }
        };

        modules.insert(name, module);
    }

    for row in input.lines() {
        let (module_str, destination_str) = row.split_once("->").unwrap();
        let destinations: Vec<String> = destination_str.split(", ").map(|d| d.trim().to_owned()).collect();
        let input = module_str.trim().to_string();
        for dest in &destinations {
            if let Some(module) = modules.get_mut(dest){
                module.register_input(&input[1..])
            }
        }
    }


    modules
}

#[derive(Debug, Clone)]
enum Pulse {
    Low,
    High
}

trait Module {
    fn evaluate(&mut self, input: &String, pulse: Pulse) -> Vec<(String, Pulse)>;

    fn register_input(&mut self, input: &str) {}

    fn print(&self);
}

#[derive(Debug)]
struct Broadcaster {
    destinations: Vec<String>
}

impl Module for Broadcaster {
    fn evaluate(&mut self, _: &String, pulse: Pulse) -> Vec<(String, Pulse)> {
        self.destinations.iter().map(|d| (d.clone(), pulse.clone())).collect()
    }

    fn print(&self) {
        println!("Broadcaster({:?})", self.destinations);
    }
}

#[derive(Debug)]
struct FlipFlop {
    destinations: Vec<String>,
    is_on: bool
}

impl Module for FlipFlop {
    fn evaluate(&mut self, _: &String, pulse: Pulse) -> Vec<(String, Pulse)> {
        if let Pulse::High = pulse {
            return Vec::new();
        }
        
        self.is_on = !self.is_on;

        let output = if self.is_on {
            Pulse::High
        } else {
            Pulse::Low
        };

        self.destinations.iter().map(|d| (d.clone(), output.clone())).collect()
    }

    fn print(&self) {
        println!("FlipFlop({:?}, {:?})", self.destinations, self.is_on);
    }
}

#[derive(Debug)]
struct Conjuction {
    inputs: HashMap<String, Pulse>,
    destinations: Vec<String>
}

impl Module for Conjuction {
    fn evaluate(&mut self, input: &String, pulse: Pulse) -> Vec<(String, Pulse)> {
        *self.inputs.get_mut(input).unwrap() = pulse;

        let all_high = self.inputs.iter().all(
            |(_,p)| match p {
                Pulse::High => true,
                Pulse::Low => false
            }
        );

        let output = if all_high {
            Pulse::Low
        } else {
            Pulse::High
        };

        self.destinations.iter().map(|d| (d.clone(), output.clone())).collect()
    }

    fn register_input(&mut self, input: &str) {
        self.inputs.insert(input.to_string(), Pulse::Low);
    }

    fn print(&self) {
        println!("Conjucation({:?}, {:?})", self.inputs, self.destinations);
    }
}

fn evaluate_once(i: i32, modules: &mut HashMap<String, Box<dyn Module>>) -> (i32, i32) {
    let mut to_do = VecDeque::from([("button".to_string(), vec![("broadcaster".to_string(), Pulse::Low)])]);
    let mut high_pulse_count = 0;
    let mut low_pulse_count = 0;

    while let Some((source, pulses)) = to_do.pop_front() {
        for (module_name, pulse) in pulses {
            match &pulse {
                Pulse::High => { high_pulse_count += 1; },
                Pulse::Low => { 
                    low_pulse_count += 1; 
                }
            }
            if let Some(module) = modules.get_mut(&module_name) {
                let output = module.evaluate(&source, pulse);
                to_do.push_back((module_name, output));
            }
            
        }
    }

    (high_pulse_count, low_pulse_count)
}



fn main() {
    let mut modules = parse_input();

    let mut total_high = 0;
    let mut total_low = 0;

    modules.get("ls").unwrap().print();
    for i in 0..1000 {
        let (high, low) = evaluate_once(i, &mut modules);
        total_high += high;
        total_low += low;
    }
    println!("{}", total_high * total_low);

    // Part 2:
    // rx connects to ls, which is a conjugator, so for rx to be sent a low signal, it musy have received high inputs from all its inputs
    // ls connects tp tx, dd, nz, ph
    // tx sends a low signal every 4051 pushes
    // dd sends a low signal every 3889 pushes
    // nz sends a low signal every 3907 pushes
    // ph sends a low signal every 3779 pushes
    // Therefore ls sends a low signal after the LCM of these which is 232,605,773,145,467 pushes
}

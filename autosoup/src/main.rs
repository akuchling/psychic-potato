
use rand::Rng;

// Convert a state # such as 1 into the matching character, like 'B'.
fn state_to_char(state: usize) -> char {
    return (('A' as u8) + (state as u8)) as char;
}

// Convert a character like 'B' into a state # like 1.
fn char_to_state(ch: char) -> usize {
    ((ch as u8) - ('A' as u8)) as usize
}

#[derive(Debug)]
struct Flib {
    num_states: usize,
    current_state: usize,
    // Each state's transition table is an input symbol, and the
    // number of the new state to transition to.
    states: Vec<Vec<(char, usize)>>,
}

impl Flib {
    fn transition(&mut self, input: char) -> char {
        // Look for matching character in the state transition table.
        // input should end up either 0 or 1.
        let input = (input as usize) - ('0' as usize);
        let (output, dest_state) = self.states[self.current_state][input];
        self.current_state = dest_state;
        return output;
    }

    fn predict(&mut self, environment: &String) -> f32 {
	let double_env = environment.to_owned() + &environment;
	let mut matches = 0;
        self.current_state = 0;
	for ch in double_env.chars() {
	     let prediction = self.transition(ch);
	     // XXX wrong!  need to shift by 1!
	     if prediction == ch {
	        matches = matches + 1;
	     }
	}


	return (matches as f32) / ((environment.len() * 2) as f32);
    }

    fn as_chromosome(&self) -> String {
        let mut c = String::from("");
        for state in &self.states {
            for transition in state {
                c.push(transition.0);
                c.push(state_to_char(transition.1));
            }
        }
        return c;
    }

    fn from_chromosome(&mut self, chromosome: String) {
        self.current_state = 0;
        // Each state occupies 4 characters, so the total number of
        // states is just the length divided by 4.
        self.num_states = (chromosome.chars().count()) / 4;

        // Fill out the state vectors
        self.states = vec![];

        let mut it = chromosome.chars();

        for _i in 0..self.num_states {
            // Get four characters from the iterator
            let output0 = it.next().expect("first character missing");
            let dest0 = it.next().expect("second character missing");
            let output1 = it.next().expect("third character missing");
            let dest1 = it.next().expect("fourth character missing");

            let transition = vec![
                (output0, char_to_state(dest0)),
                (output1, char_to_state(dest1)),
            ];
            self.states.push(transition);
        }

    }

    // XXX It would be nice to make this a class method.
    fn randomize(&mut self, num_states: usize) {
        // Create a random set of state transitions
        self.num_states = num_states;
        self.current_state = 0;
        self.states = vec![];

        for _i in 0..num_states {
            let mut new_state = vec![];
            for _j in 0..2 {
                new_state.push((
                    (('0' as u8) + rand::thread_rng().gen_range(0, 2)) as char,
                    rand::thread_rng().gen_range(0, num_states),
                ));
            }
            self.states.push(new_state);
        }
    }
}

fn output_population(heading: String, population: &Vec<Flib>) {
    println!("{}", heading);
    for flib in population {
        println!("{:?}", flib.as_chromosome());
    }
    println!("");
}

// Evaluate the entire population on how well they predict the environment
fn score_population(population: &mut Vec<Flib>, environment: &String) -> Vec<f32> {
    let mut scores: Vec<f32> = vec![];
    for flib in population {
    	scores.push(flib.predict(environment));
    }

    return scores;
}

fn simulate() -> Option<String> {
    // Sequence of symbols representing the environment
    let environment = String::from("011001");

    // Create flibs
    let population_size: i32 = 10;
    let mut population: Vec<Flib> = vec![];
    for _i in 0..population_size {
    	let mut newflib = Flib {num_states: 0, current_state: 0, states: vec![]};
	newflib.randomize(environment.len());
    	population.push(newflib);
    };

    output_population("Initial population:".to_string(), &population);
    let mut generation = 0;
    loop {
        // Score predictions based on the environment.  The score is a
	// decimal value between 0.0 and 1.0, where 1.0 is a perfect predictor
	// and 0.0 would be a perfect anti-predictor.
	let scores = score_population(&mut population, &environment);
	println!("{:?}", scores);

        // Check if we have an exact match
	if let Some(v) = find_element(scores, 1.0) {
	    return Some(population[v].as_chromosome());
	}

        // XXX Cross breed some flibs
	generation = generation + 1;

	output_population(format!("Generation {}:", generation), &population);
    }

    return None;
}

// XXX It doesn't look like the Vec class has a method which returns this.
fn find_element(vec: Vec<f32>, element: f32) -> Option<usize> {
   for i in 0..vec.len() {
       if vec[i] == element {
           return Some(i);
       }
   }
   return None;
}


fn main() {
    // Test a flib that just echoes its environment
    let mut flib = Flib {
        num_states: 1,
        current_state: 0,
        states: vec![vec![('0', 0), ('1', 0)]],
    };

    println!(
        "{} {} {}",
        flib.as_chromosome(),
        flib.transition('0'),
        flib.current_state
    );
    println!(
        "{} {} {}",
        flib.as_chromosome(),
        flib.transition('1'),
        flib.current_state
    );

    // Test a flib with two states
    let mut flib = Flib {
        num_states: 1,
        current_state: 0,
        states: vec![vec![('0', 1), ('1', 1)], vec![('1', 0), ('0', 0)]],
    };

    println!(
        "{} {} {}",
        flib.as_chromosome(),
        flib.transition('0'),
        flib.current_state
    );
    println!(
        "{} {} {}",
        flib.as_chromosome(),
        flib.transition('0'),
        flib.current_state
    );

    flib.from_chromosome(flib.as_chromosome());
    println!("{} {}", flib.as_chromosome(), flib.current_state);

    println!("{:?}", flib);
    flib.randomize(5);
    println!("{:?}", flib);

    println!("{} {}", flib.as_chromosome(), flib.current_state);
    flib.from_chromosome(flib.as_chromosome());
    println!("{} {}", flib.as_chromosome(), flib.current_state);

    let perfect = simulate();
    match perfect {
       Some(chromosome) => {println!("Perfect predictor: {}", chromosome);}
       None => {println!("No perfect predictor found");}
    }
}

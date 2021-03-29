
use std::iter::Iterator;
use rand::Rng;

// Convert the integer number `state` into the matching character; for example, 1 maps to 'B'.
fn state_to_char(state: usize) -> char {
    return (('A' as u8) + (state as u8)) as char;
}

// Convert a character `char` such as 'B' into a state # like 1.
fn char_to_state(ch: char) -> usize {
    ((ch as u8) - ('A' as u8)) as usize
}

// Structure representing a single automaton.
#[derive(Debug)]
struct Flib {
    num_states: usize,
    current_state: usize,
    // Each state's transition table is an input symbol, and the
    // number of the new state to transition to.
    states: Vec<Vec<(char, usize)>>,
}

impl Flib {

    // Consume the input character `char`, outputting a response character and
    // updating the flib's internal state according to its transition table.
    fn transition(&mut self, input: char) -> char {
        // Look for matching character in the state transition table.
        // input should end up either 0 or 1.
        let input = (input as usize) - ('0' as usize);
        let (output, dest_state) = self.states[self.current_state][input];
        self.current_state = dest_state;
        return output;
    }

    // Given an environment string, initialize the Flib to its starting state and then
    // assess how well it predicted the environment.  Returns a floating-point number
    // that's the ratio of correct predictions: 1.00 would be a perfect predictor, and 0.00
    // would be a perfect anti-predictor.
    fn predict(&mut self, environment: &String) -> f32 {
	// The predicted environment is two copies of the input
	// environment, but shifted over by one character.
	let double_env = environment.to_owned() + &environment;
	let mut predicted_env = String::new();
	{
	    let mut it = environment.chars();
	    let first_char = it.next().unwrap();
	    for ch in it {
		    predicted_env.push(ch);
	    }
	    predicted_env.push_str(&environment);  // Second copy
	    predicted_env.push(first_char);
	}

	// Feed the doubled environment into the Flib, and compare its
	// predictions to the shifted environment.
        let mut matches = 0;
        self.current_state = 0;
        for (input, expected) in double_env.chars().zip(predicted_env.chars()) {
            let prediction = self.transition(input);
            if prediction == expected {
                matches = matches + 1;
            }
        }

        return (matches as f32) / ((environment.len() * 2) as f32);
    }

    // Return a string representation of the Flib's transition table.
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

    // Replace the Flib's transition table with the table given by the specified chromosome string.
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
        // XXX could write this to produce a string and then use make_from_chromosome()
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

fn make_from_chromosome(chromosome: String) -> Flib {
    let mut baby = Flib {
        num_states: 0,
        current_state: 0,
        states: vec![],
    };
    baby.from_chromosome(chromosome);
    return baby;
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

fn random_combine(parent1: &String, parent2: &String) -> String {
    let mut result = String::new();
    let split = rand::thread_rng().gen_range(0, parent1.len());
    for (i, (ch1, ch2)) in Iterator::enumerate(Iterator::zip(parent1.chars(), parent2.chars())) {
        if i < split {
            result.push(ch1)
        } else {
            result.push(ch2);
        }
    }
    return result;
}

fn mutate_at_index(chromosome: &String, random_position: usize) -> String {
    let mut new_chromosome = String::new();

    for (i, ch) in chromosome.chars().enumerate() {
      if i != random_position {
         new_chromosome.push(ch);
      } else {
	  if (i % 2) == 0 {
	     // If we're mutating an even index, we'll flip a 0 to 1 or vice versa
	     match ch {
	       '0' => new_chromosome.push('1'),
	       '1' => new_chromosome.push('0'),
	       _ => panic!("unexpected character in chromosome"),
	     }
     	  } else {
	     // If we're mutating an odd index, we'll pick a new state
	     // We don't bother to check if we're picking the same
	     // existing state.
	     let num_states = chromosome.len() / 4;
	     let new_state = rand::thread_rng().gen_range(0, num_states);
	     new_chromosome.push(state_to_char(new_state));
	  }
      }
    }

    return new_chromosome;
}

fn mutate(chromosome: &String) -> String {
    let random_position = rand::thread_rng().gen_range(0, chromosome.len());
    return mutate_at_index(chromosome, random_position);
}


fn simulate() -> Option<String> {
    // Sequence of symbols representing the environment
    let environment = String::from("011001");

    // Create flibs
    let population_size: i32 = 10;
    let mut population: Vec<Flib> = vec![];
    for _i in 0..population_size {
        let mut newflib = Flib {
            num_states: 0,
            current_state: 0,
            states: vec![],
        };
        newflib.randomize(environment.len());
        population.push(newflib);
    }

    output_population("Initial population:".to_string(), &population);
    let mut generation = 0;
    loop {
        // Score predictions based on the environment.  The score is a
        // decimal value between 0.0 and 1.0, where 1.0 is a perfect predictor
        // and 0.0 would be a perfect anti-predictor.
        let scores = score_population(&mut population, &environment);
        println!("{:?}", scores);

        // Check if we have an exact match
        if let Some(v) = find_element(&scores, 1.0) {
            return Some(population[v].as_chromosome());
        }

        // Cross-breed the best and worst-scoring flibs, replacing
        // the worst-scoring.
        let (min_index, max_index) = find_minmax(&scores);
        println!("Worst-scoring index: {} {}", min_index, scores[min_index]);
        println!(" Best-scoring index: {} {}", max_index, scores[max_index]);
        let embryo = random_combine(
            &population[min_index].as_chromosome(),
            &population[max_index].as_chromosome(),
        );
        println!("New chromosome from crossing: {}", embryo);
        population[min_index].from_chromosome(embryo);

        let random_index = rand::thread_rng().gen_range(0, population.len());
        if random_index != max_index && random_index != min_index {
	    let subject = population[random_index].as_chromosome();
            let mutant = mutate(&subject);
            println!("Mutating #{} {} to {}", random_index, subject,
	    		       mutant);
            population[random_index].from_chromosome(mutant);
        }

        generation = generation + 1;

        output_population(format!("Generation {}:", generation), &population);
    }

    return None;
}

// XXX It doesn't look like the Vec class has a method which returns this.
fn find_element(vec: &Vec<f32>, element: f32) -> Option<usize> {
    for i in 0..vec.len() {
        if vec[i] == element {
            return Some(i);
        }
    }
    return None;
}

// Find highest and lowest scores
fn find_minmax(vec: &Vec<f32>) -> (usize, usize) {
    let mut min_index: usize = 0;
    let mut max_index: usize = 0;
    let mut min_score = 1.0;
    let mut max_score = 0.0;

    for i in 0..vec.len() {
        if vec[i] > max_score {
            max_score = vec[i];
            max_index = i;
        }
        if vec[i] < min_score {
            min_score = vec[i];
            min_index = i;
        }
    }
    return (min_index, max_index);
}

fn main() {
    let perfect = simulate();
    match perfect {
	Some(chromosome) => {
	    println!("Perfect predictor: {}", chromosome);
	}
	None => {
	    println!("No perfect predictor found");
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_flib() {
	// Test a flib that just echoes its environment
	let mut flib = Flib {
	    num_states: 1,
	    current_state: 0,
	    states: vec![vec![('0', 0), ('1', 0)]],
	};

	assert_eq!(flib.as_chromosome(), "0A1A");

	// Input a 0 and 1, and check that we get a 0 or 1 back
	assert_eq!(flib.transition('0'), '0');
	assert_eq!(flib.transition('1'), '1');
    }

    #[test]
    fn test_flib_round_trip() {
	// Test that a flib with two states round-trips to string and back.
	let mut flib = Flib {
	    num_states: 1,
	    current_state: 0,
	    states: vec![vec![('0', 1), ('1', 1)], vec![('1', 0), ('0', 0)]],
	};
	assert_eq!(flib.as_chromosome(), "0B1B1A0A");

	// After round-trip, the chromosome value should be the same
	flib.from_chromosome(flib.as_chromosome());
	assert_eq!(flib.as_chromosome(), "0B1B1A0A");
    }

    #[test]
    fn test_flib_chromosome_wrong_length() {
	// Test that supplying a chromosome whose length isn't a multiple of four will panic.
	make_from_chromosome(String::from("0A1"));
    }

    #[test]
    fn test_flib_mutation() {
	// Test that mutation at least runs without panicking.
	mutate_at_index(&String::from("0A1B1A0B"), 0);
	mutate_at_index(&String::from("0A1B1A0B"), 1);
    }

    #[test]
    fn test_two_state_flib() {
	// Test a flib with two states
	let mut flib = Flib {
	    num_states: 1,
	    current_state: 0,
	    states: vec![vec![('0', 1), ('1', 1)], vec![('1', 0), ('0', 0)]],
	};
	assert_eq!(flib.as_chromosome(), "0B1B1A0A");
	flib.transition('0');
	assert_eq!(flib.current_state, 1);
	flib.transition('0');
	assert_eq!(flib.current_state, 0);
    }

    #[test]
    fn test_randomize_method() {
	let mut flib = Flib {
	    num_states: 1,
	    current_state: 0,
	    states: vec![],
	};

	flib.randomize(5);
	assert_eq!(flib.num_states, 5);
    }
}

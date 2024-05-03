//imports i needed to use
extern crate rand;
use rand::{seq::SliceRandom, thread_rng};
use std::io::{self, Write};

//setting up necessary structure for code
struct Event {
    description: String,
    options: Vec<Choice>,
}

struct Map {
    events: Vec<Event>,
    gas_price: i8,
    vibes_price: i8,
    snacks_price: i8,
}

struct Effect {
    gas: i8,
    vibes: i8,
    snacks: i8,
    money: i8,
}

struct Choice {
    choice: String,
    outcome: String,
    effects: Effect,
    required_money: Option<i8>,
    required_snacks: Option<i8>, 
}

//the game code!!!!!
impl Map {
    fn new(events: Vec<Event>, gas_price: i8, vibes_price: i8, snacks_price: i8) -> Self {
        Map {
            events,
            gas_price,
            vibes_price,
            snacks_price,
        }
    }


    //shuffles the order of events
    fn shuffle_events(&mut self) -> Vec<usize> {
        let mut rng = thread_rng();
        let mut indices: Vec<usize> = (0..self.events.len()).collect();
        indices.shuffle(&mut rng);
        indices
    }

    //prints out shop information
    fn shop(&self, money: i8) {
        println!("Welcome to the store!");
        println!("You have {} dollars", money);
        println!("These are the prices of things:");
        println!("Gas: {} dollars per gallon", self.gas_price);
        println!("Snacks: {} dollars per snack", self.snacks_price);
        println!("Fun Activities: {} dollars per game", self.vibes_price);
    }

    fn start_game(&mut self) {

        //allows gas, vibes, and snacks to be bought at the store
        let mut gas: i8 = 0;
        let mut vibes: i8 = 0;
        let mut snacks: i8 = 0;
        let mut money: i8 = 30;

        self.shop(money);
        loop {
            loop {
                println!("Enter the number of gallons of gas you want:");
                gas = get_user_input("Gas: ");
                if gas >= 0 {
                    break;
                } else {
                    println!("Gas amount cannot be negative. Put a positive number in please.");
                }
            }
        
            loop {
                println!("Enter the number of snacks you want:");
                snacks = get_user_input("Snacks: ");
                if snacks >= 0 {
                    break;
                } else {
                    println!("Snacks amount cannot be negative. Put a positive number in please.");
                }
            }
        
            loop {
                println!("Enter the number of fun activities you want:");
                vibes = get_user_input("Fun activities: ");
                if vibes >= 0 {
                    break;
                } else {
                    println!("Fun activities amount cannot be negative. Put a positive number in please.");
                }
            }

            //does the math to check if you can afford what you bought
            let total_cost = gas * self.gas_price + snacks * self.snacks_price + vibes * self.vibes_price;
            if total_cost <= money && gas > 0 && snacks > 0 && vibes > 0 {
                println!("You have purchased {} gas, {} snacks, and {} fun activities (+{} vibes).", gas, snacks, vibes, vibes);
                money -= total_cost;
                println!("You have {} dollars left.", money);
                break;
            } else {
                println!("You must buy at least 1 of each item and not exceed your money. Please try again.");
            }
        }

        
        let shuffled_indices = self.shuffle_events();
        let mut current_event_index = 0;

        //loops through all events while game-over conditions arent true
        while current_event_index < shuffled_indices.len() && gas > 0 && vibes > 0 && snacks > 0 {
            //sets current event and prints out description
            let current_event = &self.events[shuffled_indices[current_event_index]];
            println!("{}", current_event.description);

            // prints out choices
            for (index, choice) in current_event.options.iter().enumerate() {
                //handles choices where min money is required
                if let Some(required_money) = choice.required_money {
                    if required_money > money {
                        println!("{}. {} (Requires at least {} money)", index + 1, choice.choice, required_money);
                    } else {
                        println!("{}. {} (Cost: {} money)", index + 1, choice.choice, required_money);
                    }
                } 
                //handles choices where min snacks is required
                else if let Some(required_snacks) = choice.required_snacks {
                    if required_snacks > snacks {
                        println!("{}. {} (Requires at least {} snacks)", index + 1, choice.choice, required_snacks);
                    } else {
                        println!("{}. {}", index + 1, choice.choice);
                    }
                } else {
                    println!("{}. {}", index + 1, choice.choice);
                }
            }

            // player input
            let mut choice_index;
            loop {
                choice_index = get_user_input("Input choice ") - 1; 
                if choice_index >= 0 && choice_index < current_event.options.len() as i8 {
                    //handles choice where min required money
                    if let Some(required_money) = current_event.options[choice_index as usize].required_money {
                        if required_money <= money {
                            break;
                        } else {
                            println!("You don't have enough money for this choice. Choose another.");
                        }
                    }
                    //handles choice where min required snacks 
                    else if let Some(required_snacks) = current_event.options[choice_index as usize].required_snacks {
                        if required_snacks <= snacks {
                            break;
                        } else {
                            println!("You don't have enough snacks for this choice. Choose another.");
                        }
                    } else {
                        break;
                    }
                } else {
                    println!("That's not right! Input a choice");
                }
            }

            let choice = &current_event.options[choice_index as usize];

            // print outcome
            println!("{}", choice.outcome);

            // applies choice effects
            gas += choice.effects.gas;
            vibes += choice.effects.vibes;
            snacks += choice.effects.snacks;
            money += choice.effects.money;
            
            println!("You have {} gas, {} vibes, {} snacks, and {} dollars left! \n", gas, vibes, snacks, money);

            current_event_index += 1;
        }

        //handles the end of the game
        if gas > 0 && vibes > 0 && snacks > 0{ //good endings
            println!("After a long journey, you successfully made it to the end of your road trip! Congratulations");
        }
        else{ //bad endings
            if gas <= 0 {
                println!("You ran out of gas and got stranded FOREVER!!!!");
            }
            if vibes <= 0 {
                println!("The vibes of this trip were absolutely horrific. All your friends left you!");
            }
            if snacks <= 0 {
                println!("You ran out of all your snacks, how can you continue in this state?");
            }
            println!("GAME OVER! Better luck next time.");
        }
    }
}

// get user inputs
fn get_user_input(prompt: &str) -> i8 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<i8>() {
            Ok(choice) => return choice,
            Err(_) => println!("That's not right! Input a valid number."),
        }
    }
}

fn main() {

    //structure of game
    let mut map = Map::new(
        vec![
            Event {
                description: "The next exit is a gas station. Do you want to stop and reup on supplies?".to_string(),
                options: vec![
                    Choice {
                        choice: "Keep on driving.".to_string(),
                        outcome: "You decide to keep on driving and pass the gas station.".to_string(),
                        effects: Effect {
                            gas: -1,
                            vibes: 0,
                            snacks: 0,
                            money: 0,
                        },
                        required_money: None,
                        required_snacks: None,
                    },
                    Choice {
                        choice: "Stop at the gas station.".to_string(),
                        outcome: "You buy 2 packs of snacks and 3 gallons of gas.".to_string(),
                        effects: Effect {
                            gas: 2,
                            vibes: 0,
                            snacks: 3,
                            money: -4,
                        },
                        required_money: Some(4), 
                        required_snacks: None,
                    },
                ],
            },
            Event {
                description: "After pulling over for a break, you see a squirrel next to the car looking up at you".to_string(),
                options: vec![
                    Choice {
                        choice: "Offer the squirrel some snacks".to_string(),
                        outcome: "You give the squirrel a snack. It chitters happily, runs off, and brings you back some crossword puzzles!".to_string(),
                        effects: Effect {
                            gas: -1,
                            vibes: 2,
                            snacks: -1,
                            money: 0,
                        },
                        required_money: None,
                        required_snacks: Some(2),
                    },
                    Choice {
                        choice: "Ignore the squirrel".to_string(),
                        outcome: "You turn around to get back in the car and drive away, but before you know it the squirrel is running off with some of your change!".to_string(),
                        effects: Effect {
                            gas: -1,
                            vibes: 0,
                            snacks: 0,
                            money: -5,
                        },
                        required_money: None,
                        required_snacks: None,
                    },
                ],
            },
            Event {
                description: "You see that there is a roadside attraction coming up on your route ".to_string(),
                options: vec![
                    Choice {
                        choice: "Keep driving.".to_string(),
                        outcome: "You decide to keep on driving and pass the roadside attraction, to the dismay of your passengers.".to_string(),
                        effects: Effect {
                            gas: -1,
                            vibes: -1,
                            snacks: -1,
                            money: 0,
                        },
                        required_money: None,
                        required_snacks: None,
                    },
                    Choice {
                        choice: "Stop at the roadside attraction".to_string(),
                        outcome: "You decide to stop and have a great time walking around the overpriced wacky museum. What memories!".to_string(),
                        effects: Effect {
                            gas: -1,
                            vibes: 3,
                            snacks: 0,
                            money: -4,
                        },
                        required_money: Some(4), 
                        required_snacks: None,
                    },
                ],
            },
            Event {
                description: "As you drive around dinner, you pass by a small old restaurant.".to_string(),
                options: vec![
                    Choice {
                        choice: "Keep driving and eat your snacks. ".to_string(),
                        outcome: "You decide to keep on driving and just eat your snacks. Cost-efficient but less fun.".to_string(),
                        effects: Effect {
                            gas: -1,
                            vibes: -2,
                            snacks: -2,
                            money: 0,
                        },
                        required_money: None,
                        required_snacks: None,
                    },
                    Choice {
                        choice: "Stop at the restaurant.".to_string(),
                        outcome: "You decide to stop at the restauraunt and have some of the best food of your life. Worth it!!!".to_string(),
                        effects: Effect {
                            gas: -1,
                            vibes: 3,
                            snacks: 1,
                            money: -4,
                        },
                        required_money: Some(4), 
                        required_snacks: None,
                    },
                ],
            },
            Event {
                description: "Your car just broke down! How do you fix it?".to_string(),
                options: vec![
                    Choice {
                        choice: "Fix it with your passengers".to_string(),
                        outcome: "You and your passengers spend a solid few hours figuring out how to fix the car and feel really exhausted now.".to_string(),
                        effects: Effect {
                            gas: -1,
                            vibes: -2,
                            snacks: -1,
                            money: 0,
                        },
                        required_money: None,
                        required_snacks: None,
                    },
                    Choice {
                        choice: "Call a mechanic".to_string(),
                        outcome: "You decide to call a mechanic to fix the car. ".to_string(),
                        effects: Effect {
                            gas: -1,
                            vibes: 0,
                            snacks: 0,
                            money: -3,
                        },
                        required_money: Some(3), 
                        required_snacks: None,
                    },
                ],
            },
            Event {
                description: "You guys just crossed the state line! How do you celebrate?".to_string(),
                options: vec![
                    Choice {
                        choice: "Collect some dirt from the new state".to_string(),
                        outcome: "You put some dirt in a jar as a keepsake. How sentimental!".to_string(),
                        effects: Effect {
                            gas: -1,
                            vibes: 2,
                            snacks: 0,
                            money: 0,
                        },
                        required_money: None,
                        required_snacks: None,
                    },
                    Choice {
                        choice: "Stop for slushies".to_string(),
                        outcome: "You decide to stop for some slushies to see if they taste the same across state lines.".to_string(),
                        effects: Effect {
                            gas: -1,
                            vibes: 0,
                            snacks: 2,
                            money: -3,
                        },
                        required_money: Some(3), 
                        required_snacks: None,
                    },
                ],
            },
        ],
        2,  
        2,  
        1,  
    );

    // starts the game
    map.start_game(); 
}





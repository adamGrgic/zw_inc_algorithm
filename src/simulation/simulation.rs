use rand::Rng;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;
use std::fs;
use std::error::Error;
use rusqlite::{params, Connection, Result};

use crate::models::SimulationFrame; // Bring models into scope
use crate::models::SimulationResult; // Bring models into scope
//
//

// fn check_root_dir() -> io::Result<()> {
//
//
//     // Serialize results to JSON
//     let json = serde_json::to_string_pretty(&results).unwrap();
//
//     let simulation_results_file = &format!( "results_{}.json",path,simulation_id);
//
//
//
//     // Write to a file
//     let mut file = File::create(simulation_results_file)?;
//     file.write_all(json.as_bytes())?;
//
//
//     Ok(())
// }
//
struct SimulationConfig {
    process_id: Uuid,
    simulations: i32,
    ticks: i32,
    maximum_upgrades: i32,
    income_interval: i32,
    initial_balance: i32
}

fn start_simulation(config: SimulationConfig)-> Vec<SimulationResult> {
    // feels these config values should be a layer up
    let num_simulations = 1000; // Number of simulations
    let total_ticks = 1500; // Simulate 600 seconds (10 minutes)

    // starting conditions
    let maximum_upgrades = 50;
    let income_interval = 18;
    let initial_balance = 500;


    let simulation_results: Vec<SimulationResult> = (0..num_simulations)
        .into_iter()
        .map(|id| {

            let income_upgrade_amount = 2;

            let mut rng = rand::thread_rng();
            let simulation_id = Uuid::new_v4();

            let mut income = 50;
            let mut income_upgrade_counts = maximum_upgrades.clone();
            let mut income_upgrade_cost = 50;
            let mut balance = initial_balance.clone();

            // idea: create random selector module (r_select)
            // given 33% chance of picking x , give the inputs of the
            // probability (1/3) and the item in question
            //
            // if its something like 33, then create an array of 100 items
            // mark 33 items as true and 67 as false
            //
            // .. hmm but what if its 33% chance of picking 1 item out of 100 unique items,
            // how would I weigh that?
            //
            // could you multiply the total items (100) by the percentage criteria (33)
            // which would give you 3300
            // and then mark 33% of those items as possible true selections
            // distribute the rest evenly?
            //
            // so multiply 3300 by .33 which gives us 1,089
            // then, subtract 1,089 from 3300 which gives us 2,221
            // then, take 2,221 divide it into 99 equal parts which gives us
            // 22.34 if we round to the tenth decimal place
            // so then to weigh an item as 33% chance of being selected and knowing which item
            // was selected at 'runtime', we would create an array
            // of 3300 items (we'll say of type Selection {id: "name of item here", isWeightedItem: bool} , 1,089 of them with id's of our weighted item and then add 22 of the other items

            let mut simulation_frames: Vec<SimulationFrame> = Vec::with_capacity(total_ticks);

            for tick in 0..total_ticks {
                let frame_id = tick as i32; // Treat tick as x
                let random_number = rng.gen_range(0..10);

                if frame_id % income_interval == 0 {
                    balance += income;
                }


                let secret_number = 3;

                if random_number == secret_number {
                    // simulating player's RNG on getting money
                    let random_balance_addition = rng.gen_range(32..400);

                    balance += random_balance_addition;
                }


                if balance > income_upgrade_cost && income_upgrade_counts > 0 {
                    income_upgrade_counts -= 1;
                    income_upgrade_cost += income_upgrade_amount;
                    income += income_upgrade_amount;
                    balance -= income_upgrade_cost;
                }


                let simulation_frame = SimulationFrame {
                    simulation_id,
                    income,
                    balance,
                    income_upgrade_cost,
                    income_upgrade_counts,
                    frame_id
                };

                simulation_frames.push(simulation_frame);
            }

            SimulationResult {
                simulation_id: config.process_id,
                values: simulation_frames,
            }
        })
        .collect();

    simulation_results

}

pub fn create_simulation()->Result<(),Box<dyn Error>> {
    println!("Generating stats simulation...");

    println!("Checking to make sure results directory exists...");

    let project_root = std::env::current_dir().expect("Failed to get current directory");
    let results_path = project_root.join("target/generated");


    match fs::create_dir_all(results_path.clone()) {
        Ok(_) => {
            println!("Successfully created directory: {}", results_path.display());
        }
        Err(e) => {
            eprintln!("Failed to create directory: {}. Error: {}", results_path.display(), e);
            // Handle the error appropriately (e.g., return it or take recovery steps)
            return Err(Box::new(e));
        }
    }

    let process_id = Uuid::new_v4();
    let simulation_config = SimulationConfig {
        process_id,
        simulations: 1000,
        ticks: 1500,
        maximum_upgrades: 50,
        income_interval: 18,
        initial_balance: 500

    };

    let results = start_simulation(simulation_config);

    // Serialize results to JSON
    let json = serde_json::to_string_pretty(&results).unwrap();

    let simulation_results_file = &format!( "results_{}.json",process_id.clone());



    // Write to a file
    let mut file = File::create(results_path.join(simulation_results_file))?;
    file.write_all(json.as_bytes())?;

    let conn = Connection::open("simulation_frames.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )",
        [],
    )?;

    // Insert a new user
    conn.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        params!["Alice", 30],
    )?;

    let _ = conn.close();



    // let conn2 = Connection::open("simulation.db")

    println!("Data inserted successfully!");

    println!("Simulation complete! Results written to 'results.json'.");
    Ok(())

}


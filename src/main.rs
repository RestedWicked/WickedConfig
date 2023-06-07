use linked_hash_map::LinkedHashMap;
use std::fs::File;
//use std::io::prelude::*;
use std::println;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct LootboxRewards {
    name: String,
    give_command: Vec<String>,

    #[serde(default = "String::new")]
    message_command: String,

    material: String,
    repeats: u32,
    initial_amount: u32,
    maximum_amount: u32,
    steps: usize,
    reward_type: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct LootboxHasSections {
    value0: u32,
    compare0: String,
    material: String,
    stack: u8,
    name: String,
    commands: Vec<String>,
}

fn main() -> Result<(), serde_yaml::Error> {
    let yaml_file = File::open("INPUT/template.yml").expect("Owo I cannot read this. Uwu");

    let deserialized_yaml: LinkedHashMap<u32, LootboxRewards> =
        serde_yaml::from_reader(&yaml_file).expect("Owo I cannot read the values. Uwu");

    let mut output: LinkedHashMap<String, LootboxHasSections> = LinkedHashMap::new();

    let mut counter: u32 = 0;

    for (_key, value) in deserialized_yaml.iter() {
        for amount in (value.initial_amount..value.maximum_amount).step_by(value.steps) {
            let has_key: String = "has".to_string() + &counter.to_string();

            let mut give_commands = value.give_command.clone();
            give_commands[0] = give_commands[0].replace("{amount}", &amount.to_string());

            let has_section = LootboxHasSections {
                value0: counter + 1,
                compare0: "%cp-data-chance%".to_string(),
                material: value.material.clone(),
                stack: 1,
                name: value.name.clone().replace("{amount}", &amount.to_string()),
                commands: give_commands,
            };

            output.insert(has_key, has_section);

            //println!("{}: \n  Amount: {}", has_key, amount);
            counter += 1;
        }
    }

    let serialized_yaml = serde_yaml::to_string(&output)?;
    println!("{}", serialized_yaml);
    Ok(())
}

use dotenv;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::{self};

mod lib;

fn main() -> Result<(), Box<dyn Error>> {
    let knowledges = reify_knowledge()?;
    let speech = get_speech_json(dotenv::var("SPEECH_JSON_PATH").unwrap())?;

    let mut bob = lib::game_master::GameMaster {
        knowledges,
        speech,
        actual_creation_step: "races".to_owned(),
        actual_character: lib::knowledge::Character::new(),
    };

    bob.welcome();
    while !bob.actual_character.is_complete() {
        bob.introduce_part();
        let user_input = await_user_input()?;
        bob.evaluate(user_input);
    }
    bob.goodbye();

    Ok(())
}

fn reify_knowledge() -> Result<lib::knowledge::Encyclopedia, Box<dyn Error>> {
    let races = get_knowledge_json(dotenv::var("RACES_JSON_PATH").unwrap(), "races")?;
    let classes = get_knowledge_json(dotenv::var("CLASSES_JSON_PATH").unwrap(), "classes")?;
    let backgrounds =
        get_knowledge_json(dotenv::var("BACKGROUNDS_JSON_PATH").unwrap(), "backgrounds")?;

    let mut knowledges: lib::knowledge::Encyclopedia = lib::knowledge::Encyclopedia::new();

    knowledges.insert(String::from("races"), races);
    knowledges.insert(String::from("classes"), classes);
    knowledges.insert(String::from("backgrounds"), backgrounds);

    Ok(knowledges)
}

fn get_knowledge_json(
    path: String,
    knowledge_type: &str,
) -> Result<Vec<Box<dyn lib::knowledge::Knowable>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut knowledge: Vec<Box<dyn lib::knowledge::Knowable>> = Vec::new();
    if knowledge_type == "races" {
        let races: Vec<Box<lib::knowledge::Race>> = serde_json::from_reader(reader)?;
        for race in races.into_iter() {
            knowledge.push(race as Box<dyn lib::knowledge::Knowable>);
        }
    } else if knowledge_type == "classes" {
        let classes: Vec<Box<lib::knowledge::Classe>> = serde_json::from_reader(reader)?;
        for classe in classes.into_iter() {
            knowledge.push(classe as Box<dyn lib::knowledge::Knowable>);
        }
    } else if knowledge_type == "backgrounds" {
        let backgrounds: Vec<Box<lib::knowledge::Background>> = serde_json::from_reader(reader)?;
        for background in backgrounds.into_iter() {
            knowledge.push(background as Box<dyn lib::knowledge::Knowable>);
        }
    }

    knowledge.sort_by_key(|x| *x.get_id());

    Ok(knowledge)
}

fn get_speech_json(path: String) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let speech: HashMap<String, String> = serde_json::from_reader(reader)?;

    Ok(speech)
}

fn await_user_input() -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    let input_handler = io::stdin();
    input_handler.read_line(&mut buffer)?;

    Ok(buffer)
}

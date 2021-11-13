use character_creator::GameMaster;
use character_creator::Knowledge;
use dotenv;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::{self};

// use character_creator::Character;

fn main() -> Result<(), Box<dyn Error>> {
    let knowledge = reify_knowledge()?;
    let speech = get_speech_json(dotenv::var("SPEECH_JSON_PATH").unwrap())?;

    let bob = GameMaster {
        knowledge,
        speech
    };
    
    bob.welcome();

    bob.introduce_part(String::from("races"));
    let user_input = await_user_input()?;
    // bob.evaluate(user_input);

    // bob.introduce_part(String::from("classes"));

    // let user_input = await_user_input()?;
    

    // println!("{}", user_input);

    Ok(())
}

fn reify_knowledge() -> Result<HashMap<String, Knowledge>, Box<dyn Error>> {
    let races = get_knowledge_json(dotenv::var("RACES_JSON_PATH").unwrap(), "races")?;
    let classes = get_knowledge_json(dotenv::var("CLASSES_JSON_PATH").unwrap(), "classes")?;

    let mut knowledge: HashMap<String, Knowledge> = HashMap::new();

    knowledge.insert(String::from("races"), races);
    knowledge.insert(String::from("classes"), classes);

    Ok(knowledge)
}

fn get_knowledge_json(path: String, knowledge_type: &str) -> Result<Knowledge, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let knowledge = match knowledge_type {
        "races" => Knowledge::Races(serde_json::from_reader(reader)?),
        "classes" => Knowledge::Classes(serde_json::from_reader(reader)?),
        _ => Knowledge::None,
    };

    Ok(knowledge)
}

fn get_speech_json(path:String) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let speech : HashMap<String, String> = serde_json::from_reader(reader)?;
    
    Ok(speech)
}

fn await_user_input() -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    let input_handler = io::stdin();
    input_handler.read_line(&mut buffer)?;

    Ok(buffer)
}

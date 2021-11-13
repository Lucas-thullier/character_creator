// use character_creator::GameMaster;
use character_creator::Classe;
use character_creator::Knowable;
use character_creator::Race;
use character_creator::Knowledge;
use std::collections::HashMap;
use std::error::Error;
// use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::{self};

// use character_creator::Character;

fn main() -> Result<(), Box<dyn Error>> {
    let knowledge = reify_knowledge()?;
    // let bob = GameMaster {
    //     knowledge: knowledge,
    // };
    // bob.welcome();
    // bob.introduce_races();

    // let user_input = await_user_input()?;

    // bob.evaluate(user_input);

    // println!("{}", user_input);

    Ok(())
}

fn reify_knowledge() -> Result<HashMap<String, Knowledge>, Box<dyn Error>> {
    let races = get_knowledge_json("./storage/dnd/races.json", "races")?;
    let classes = get_knowledge_json("./storage/dnd/classes.json", "classes")?;

    let mut knowledge: HashMap<String, Knowledge> = HashMap::new();

    knowledge.insert(String::from("races"), races);
    knowledge.insert(String::from("classes"), classes);

    Ok(knowledge)
}

fn get_knowledge_json(path: &str, knowledge_type: &str) -> Result<Knowledge, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let knowledge = match knowledge_type {
        "races" => Knowledge::Race(serde_json::from_reader(reader)?),
        "classes" => Knowledge::Classe(serde_json::from_reader(reader)?),
        _ => Knowledge::None
    };

    Ok(knowledge)
}

fn await_user_input() -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    let input_handler = io::stdin();
    input_handler.read_line(&mut buffer)?;

    Ok(buffer)
}

use super::knowledge::Encyclopedia;
use std::collections::HashMap;

pub struct GameMaster {
  pub knowledges: Encyclopedia,
  pub speech: HashMap<String, String>,
  pub actual_creation_step: String,
}

impl GameMaster {
  pub fn welcome(&self) {
    println!("{}", &self.speech["welcome"]);
  }
  
  pub fn introduce_part(&self, part_name: String) {
    println!("{}", &self.speech[&part_name]);

    let knowledges = &self.knowledges[&part_name];
    for knowledge in knowledges {
      println!("{}: {}", knowledge.get_id(), knowledge.get_name())
    }
  }

  pub fn evaluate(&self, user_input: String) {
    let is_int: bool = user_input.trim().parse::<u16>().is_err();
    if is_int {}
    println!("{}", user_input)
  }

  pub fn goodbye(&self) {
    println!("{}", &self.speech["goodbye"]);
  }
}

use super::knowledge::Character;
use super::knowledge::Encyclopedia;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GameMaster {
  pub knowledges: Encyclopedia,
  pub speech: HashMap<String, String>,
  pub actual_creation_step: String,
  pub actual_character: Character,
}

impl GameMaster {
  pub fn welcome(&self) {
    println!("{}", &self.speech["welcome"]);
  }

  pub fn introduce_part(&self) {
    println!("{}", &self.speech[&self.actual_creation_step]);

    let knowledges = &self.knowledges[&self.actual_creation_step];
    for knowledge in knowledges {
      println!("{}: {}", knowledge.get_id(), knowledge.get_name())
    }
  }

  pub fn evaluate(&mut self, user_input: String) {
    let is_int: bool = !user_input.trim().parse::<u16>().is_err();

    if is_int {
      let user_input = user_input.trim().parse::<usize>().unwrap() - 1;
      let knowledge_vec = &self.knowledges[&self.actual_creation_step];
      let knowable = &knowledge_vec[user_input];

      let knowable_name = knowable.get_name().to_string();

      // let character_modifiers = Box::new(knowable.downcast.get_modifiers());
      
      knowable.get_modifiers().apply_modifiers(&mut self.actual_character);
      self.set_appropriate_field(knowable_name);
      dbg!(&self.actual_character);
    }

    self.determine_actual_step();
  }

  pub fn goodbye(&self) {
    println!("{}", &self.speech["goodbye"]);
  }

  fn determine_actual_step(&mut self) {
    if self.actual_character.race.is_empty() {
      self.actual_creation_step = "races".to_owned()
    } else if self.actual_character.class.is_empty() {
      self.actual_creation_step = "classes".to_owned()
    } else if self.actual_character.background.is_empty() {
      self.actual_creation_step = "backgrounds".to_owned()
    } else {
      self.actual_creation_step = "end".to_owned()
    }
  }

  fn set_appropriate_field(&mut self, knowable_name: String) {
    if self.actual_creation_step == "races".to_owned() {
      self.actual_character.set_race(knowable_name);
    } else if self.actual_creation_step == "classes".to_owned() {
      self.actual_character.set_class(knowable_name)
    } else if self.actual_creation_step == "backgrounds".to_owned() {
      self.actual_character.set_background(knowable_name)
    }
  }
}

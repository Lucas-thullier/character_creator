use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct GameMaster {
  pub knowledge: HashMap<String, Knowledge>,
  pub speech: HashMap<String, String>,
}

impl GameMaster {
  pub fn welcome(&self) {
    println!("{}", &self.speech["welcome"]);
  }

  pub fn introduce_part(&self, part_name: String) {
    println!("{}", &self.speech[&part_name]);

    if let Knowledge::Races(races) = &self.knowledge[&part_name] {
      let mut races = races.to_vec();
      races.sort_by_key(|x| x.id);
      for race in races {
        println!("{}: {}", race.id, race.name)
      }
    } else if let Knowledge::Classes(classes) = &self.knowledge[&part_name] {
      let mut classes = classes.to_vec();
      classes.sort_by_key(|x| x.id);
      for classe in classes {
        println!("{}: {}", classe.id, classe.name)
      }
    }
  }

  pub fn evaluate(&self, user_input: String) {
    let is_int: bool = user_input.trim().parse::<u16>().is_err();

    if is_int {}

    println!("{}", user_input)
  }

  pub fn select_subrace() {}
}

#[derive(Debug)]
pub enum Knowledge {
  Races(Vec<Box<Race>>),
  Classes(Vec<Box<Classe>>),
  None,
}

pub trait Knowable {
  fn become_knowable(self: Self) -> Box<dyn Knowable>;
  fn test(&self);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Race {
  id: u64,
  name: String,
  bonuses: CharacterModifiers,
}

impl Knowable for Race {
  fn become_knowable(self: Self) -> Box<dyn Knowable> {
    Box::new(self)
  }
  fn test(&self) {
    println!("{:?}", self.name)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Classe {
  id: u64,
  name: String,
  bonuses: CharacterModifiers,
}

impl Knowable for Classe {
  fn become_knowable(self: Self) -> Box<dyn Knowable> {
    Box::new(self)
  }
  fn test(&self) {
    println!("{:?}", self.name)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CharacterModifiers {
  ability: HashMap<String, i64>,
  proficiency: HashMap<String, Vec<String>>,
  features: Vec<String>,
  spells: Vec<String>,
  movespeed: i64,
}

// pub struct Character {
//   name: String,
//   age: u64,
//   background: String,
//   alignment: String,
//   class: String,
//   race: String,
//   level: u64,
//   experience: u64,
//   ability: Vec<Ability>,
//   skills: Vec<Skill>,
//   armor_class: ArmorClass,
//   speed: u64,
//   hit_point: HitPoint,
//   hit_dice: Vec<u16>,
//   proficiency_bonus: i16,
// }

// struct Ability {}

// struct Skill {}

// struct ArmorClass {}

// struct HitPoint {}

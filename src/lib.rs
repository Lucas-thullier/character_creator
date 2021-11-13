use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct GameMaster {
  pub knowledge: HashMap<String, Knowledge>,
}

impl GameMaster {
  pub fn welcome(&self) {
    println!("\n");
    println!("Bonjour !");
    println!("\n");
  }

  pub fn introduce_races(&self) {
    println!("Quelle race souhaitez-vous incarner?");
    println!("\n");

    if let Knowledge::Races(races) = &self.knowledge["races"] {
      for race in races {
        println!("{}: {}", race.id, race.name) 
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

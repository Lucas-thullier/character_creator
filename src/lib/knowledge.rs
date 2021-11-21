use serde::Deserialize;
use std::collections::HashMap;

pub type Encyclopedia = HashMap<String, Vec<Box<dyn Knowable>>>;

pub trait Knowable: std::fmt::Debug {
  fn get_id(&self) -> &u64;
  fn get_name(&self) -> &String;
}

#[derive(Deserialize, Debug)]
pub struct Race {
  pub id: u64,
  pub name: String,
  bonuses: CharacterModifiers,
}

impl Knowable for Race {
  fn get_id(&self) -> &u64 {
    &self.id
  }

  fn get_name(&self) -> &String {
    &self.name
  }
}

#[derive(Deserialize, Debug)]
pub struct Classe {
  pub id: u64,
  pub name: String,
  bonuses: CharacterModifiers,
}

impl Knowable for Classe {
  fn get_id(&self) -> &u64 {
    &self.id
  }

  fn get_name(&self) -> &String {
    &self.name
  }
}

#[derive(Deserialize, Debug)]
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

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
  bonuses: RaceModifiers,
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
  bonuses: ClassModifiers,
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
pub struct Background {
  pub id: u64,
  pub name: String,
  bonuses: BackgroundModifiers,
}

impl Knowable for Background {
  fn get_id(&self) -> &u64 {
    &self.id
  }

  fn get_name(&self) -> &String {
    &self.name
  }
}

// trait CharacterModifiers {
//   pub fn 
// }

#[derive(Deserialize, Debug)]
struct RaceModifiers {
  abilities: HashMap<String, i64>,
  proficiencies: HashMap<String, Vec<String>>,
  features: Vec<String>,
  spells: Vec<String>,
  movespeed: i64,
}

#[derive(Deserialize, Debug)]
struct ClassModifiers {
  hit_dice: String,
  hit_points: String,
  proficiencies: HashMap<String, Vec<String>>,
  equipments: HashMap<String, Vec<String>>,
  spells: Vec<String>,
  features: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct BackgroundModifiers {
  proficiencies: HashMap<String, Vec<String>>,
  equipments: HashMap<String, Vec<String>>,
  features: Vec<String>,
}

#[derive(Default, Debug)]
pub struct Character {
  pub name: String,
  pub age: u64,
  pub background: String,
  pub alignment: String,
  pub class: String,
  pub race: String,
  pub level: u64,
  pub experience: u64,
  pub abilities: Vec<Ability>,
  pub skills: Vec<Skill>,
  pub armor_class: ArmorClass,
  pub speed: u64,
  pub hit_point: HitPoint,
  pub hit_dice: Vec<u16>,
  pub proficiency_bonus: i16,
}

impl Character {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn set_class(&mut self, new_class: String) {
    self.class = new_class;
  }

  pub fn set_race(&mut self, new_race: String) {
    self.race = new_race;
  }

  pub fn set_background(&mut self, new_background: String) {
    self.background = new_background;
  }

  pub fn is_complete(&self) -> bool {
    if !self.race.is_empty() && !self.class.is_empty() && !self.background.is_empty() {
      true
    } else {
      false
    }
  }

  // pub fn apply_build_block(&self, character_modifiers: CharacterModifiers) {
    // character_modifiers.ability.iter()
  // }
}

#[derive(Default, Debug)]
pub struct Ability {}

#[derive(Default, Debug)]
pub struct Skill {}

#[derive(Default, Debug)]
pub struct ArmorClass {}

#[derive(Default, Debug)]
pub struct HitPoint {}

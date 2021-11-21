use serde::Deserialize;
use std::collections::HashMap;

pub type Encyclopedia = HashMap<String, Vec<Box<dyn Knowable>>>;

pub trait Knowable: std::fmt::Debug {
  fn get_id(&self) -> &u64;
  fn get_name(&self) -> &String;
  fn get_modifiers(&self) -> Box<dyn CharacterModifiers>;
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

  fn get_modifiers(&self) -> Box<dyn CharacterModifiers> {
    let bonuses = Box::new(self.bonuses.clone());

    let boxed_modifiers = bonuses as Box<dyn CharacterModifiers>;
    boxed_modifiers
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

  fn get_modifiers(&self) -> Box<dyn CharacterModifiers> {
    let bonuses = Box::new(self.bonuses.clone());

    let boxed_modifiers = bonuses as Box<dyn CharacterModifiers>;
    boxed_modifiers
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

  fn get_modifiers(&self) -> Box<dyn CharacterModifiers> {
    let bonuses = Box::new(self.bonuses.clone());

    let boxed_modifiers = bonuses as Box<dyn CharacterModifiers>;
    boxed_modifiers
  }
}

pub trait CharacterModifiers {
  fn apply_modifiers(&self, character: &mut Character) -> ();
}

#[derive(Deserialize, Debug, Clone)]
struct RaceModifiers {
  abilities: HashMap<String, i16>,
  proficiencies: HashMap<String, Vec<String>>,
  features: Vec<String>,
  spells: Vec<String>,
  movespeed: i64,
}

impl CharacterModifiers for RaceModifiers {
  fn apply_modifiers(&self, character: &mut Character) {
    for (name, value) in &self.abilities {
      if let Some(x) = character.abilities.get_mut(name) {
        x.set_race_modifier(*value)
      };
    }

    for (name, value) in &self.proficiencies {
      character
        .proficiencies
        .insert(name.to_string(), value.to_vec());
    }

    for feature in &self.features {
      character.features.push(feature.to_string());
    }

    for spell in &self.spells {
      character.spells.push(spell.to_string());
    }

    character.movespeed = self.movespeed.to_owned();
  }
}

#[derive(Deserialize, Debug, Clone)]
struct ClassModifiers {
  hit_dice: String,
  hit_points: String,
  proficiencies: HashMap<String, Vec<String>>,
  equipments: HashMap<String, Vec<String>>,
  spells: Vec<String>,
  features: Vec<String>,
}

impl CharacterModifiers for ClassModifiers {
  fn apply_modifiers(&self, character: &mut Character) {
    character.hit_dice = self.hit_dice.to_string();
    character.hit_points = self.hit_points.to_string();

    for (name, value) in &self.proficiencies {
      character
        .proficiencies
        .insert(name.to_string(), value.to_vec());
    }
    // for (name, value) in &self.abilities {
    //   if let Some(x) = character.abilities.get_mut(name) {
    //     x.set_race_modifier(*value)
    //   };
    // character.abilities[name].set_race_modifier(*value);
    // }

    // for (name, value) in &self.proficiencies {
    //   character.proficiencies[name].set_proficiencies(*value)
    // }
  }
}

#[derive(Deserialize, Debug, Clone)]
struct BackgroundModifiers {
  proficiencies: HashMap<String, Vec<String>>,
  equipments: HashMap<String, Vec<String>>,
  features: Vec<String>,
}

impl CharacterModifiers for BackgroundModifiers {
  fn apply_modifiers(&self, character: &mut Character) {
    // for (name, value) in &self.abilities {
    //   if let Some(x) = character.abilities.get_mut(name) {
    //     x.set_race_modifier(*value)
    //   };
    // character.abilities[name].set_race_modifier(*value);
    // }

    // for (name, value) in &self.proficiencies {
    //   character.proficiencies[name].set_proficiencies(*value)
    // }
  }
}

#[derive(Debug)]
pub struct Character {
  pub name: String,
  pub age: u64,
  pub background: String,
  pub alignment: String,
  pub class: String,
  pub race: String,
  pub level: u64,
  pub experience: u64,
  pub abilities: HashMap<String, Ability>,
  pub proficiencies: HashMap<String, Vec<String>>,
  pub skills: Vec<Skill>,
  pub spells: Vec<String>,
  pub features: Vec<String>,
  pub armor_class: ArmorClass,
  pub movespeed: i64,
  pub hit_points: String,
  pub hit_dice: String,
  pub proficiency_bonus: i16,
}

impl Character {
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

impl Default for Character {
  fn default() -> Self {
    let mut abilities: HashMap<String, Ability> = HashMap::new();
    abilities.insert("strength".to_owned(), Ability::default());
    abilities.insert("dexterity".to_owned(), Ability::default());
    abilities.insert("constitution".to_owned(), Ability::default());
    abilities.insert("intelligence".to_owned(), Ability::default());
    abilities.insert("wisdom".to_owned(), Ability::default());
    abilities.insert("charisma".to_owned(), Ability::default());
    let proficiencies: HashMap<String, Vec<String>> = HashMap::new();
    let skills: Vec<Skill> = Vec::new();
    let features: Vec<String> = Vec::new();
    let spells: Vec<String> = Vec::new();

    Character {
      name: "".to_owned(),
      age: 0,
      background: "".to_owned(),
      alignment: "".to_owned(),
      class: "".to_owned(),
      race: "".to_owned(),
      level: 0,
      experience: 0,
      abilities: abilities,
      proficiencies: proficiencies,
      skills: skills,
      features: features,
      spells: spells,
      armor_class: ArmorClass {},
      movespeed: 0,
      hit_points: "".to_owned(),
      hit_dice: "".to_owned(),
      proficiency_bonus: 0,
    }
  }
}

#[derive(Default, Debug)]
pub struct Ability {
  natural_value: i16,
  race_modifier: i16,
  temporary_modifier: i16,
}

impl Ability {
  pub fn value(&self) -> i16 {
    self.natural_value + self.race_modifier + self.temporary_modifier
  }

  pub fn set_race_modifier(&mut self, race_modifier: i16) {
    self.race_modifier = race_modifier
  }
}

#[derive(Default, Debug)]
pub struct Skill {}

#[derive(Default, Debug)]
pub struct ArmorClass {}

#[derive(Default, Debug)]
pub struct HitPoint {}

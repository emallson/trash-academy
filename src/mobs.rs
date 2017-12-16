use std::collections::HashMap;
use askama::Template;
use separator::Separatable;
use slug::slugify;
use serde_yaml::from_reader;
use std::fs::File;

const LEVELS: &[usize] = &[1, 10, 20];

#[derive(Template, Default)]
#[template(path = "main.html")]
pub(crate) struct MainTemplate {}

#[derive(Deserialize)]
pub struct Mob {
    pub id: usize,
    pub hp: usize,
    #[serde(default)]
    pub hp_template: String,
    pub name: String,
    pub contribution: usize,
    pub pct_contribution: String,
    pub pct_teeming_contribution: String,
    pub abilities: Vec<Ability>,
}

#[derive(Deserialize)]
pub struct Ability {
    pub id: usize,
    pub name: String,
    pub base_description: String,
    /// Contains the mythic +0 amounts of damage/healing/whatever done by the ability
    pub amounts: Vec<usize>,
    #[serde(default)]
    pub description: String,
}

impl Ability {
    // fn new(id: usize, name: &'a str, base_description: &'a str, amounts: &'a [usize]) -> Self {
    // let mut ab = Ability {
    // id,
    // name,
    // base_description,
    // amounts: Vec::from(amounts),
    // description: "".to_owned(),
    // };
    // ab.description = ab.describe();
    // ab
    // }

    pub fn describe(&self) -> String {
        let mut owned = self.base_description.to_owned();
        for &amount in &self.amounts {
            let (low, mid, high) = scale(amount, LEVELS);
            let tpl = AmountTemplate {
                amount_base: low,
                amount_mid: mid,
                amount_high: high,
            }.render()
                .unwrap();

            owned = owned.replace("%d", &tpl);
        }

        owned
    }

    pub fn self_describe(&mut self) {
        self.description = self.describe();
    }
}

#[derive(Template)]
#[template(path = "amount3.html")]
struct AmountTemplate {
    amount_base: usize,
    amount_mid: usize,
    amount_high: usize,
}

fn scale(amount: usize, levels: &[usize]) -> (usize, usize, usize) {
    assert_eq!(levels.len(), 3);
    let s = |level| (amount as f64 * 1.1f64.powi(level as i32 - 1)).ceil() as usize;
    (s(levels[0]), s(levels[1]), s(levels[2]))
}

#[derive(Template)]
#[template(path = "mob_solo.html")]
pub(crate) struct MobTemplate<'a> {
    mob: &'a Mob,
    _parent: MainTemplate,
}

impl<'a> MobTemplate<'a> {
    pub fn new(mob: &'a Mob) -> Self {
        MobTemplate {
            mob,
            _parent: MainTemplate::default(),
        }
    }
}

pub struct MobIndex {
    pub(crate) mobs: Vec<Mob>,
    pub(crate) by_id: HashMap<String, usize>,
    pub(crate) by_name: HashMap<String, usize>,
}

impl MobIndex {
    pub fn by_id(&self, val: &str) -> Option<&Mob> {
        self.by_id.get(val).map(|&u| &self.mobs[u])
    }

    pub fn by_name(&self, val: &str) -> Option<&Mob> {
        self.by_name.get(val).map(|&u| &self.mobs[u])
    }
}

pub fn load_mobs() -> MobIndex {
    let mut mobs: Vec<Mob> = from_reader(File::open("data/mobs_melded.yaml").unwrap()).unwrap();
    for mob in &mut mobs {
        for ability in &mut mob.abilities {
            ability.self_describe();
        }
        let (low, mid, high) = scale(mob.hp, LEVELS);
        let tpl = AmountTemplate {
            amount_base: low,
            amount_mid: mid,
            amount_high: high,
        }.render()
            .unwrap();
        mob.hp_template = tpl;
    }
    let by_id = mobs.iter()
        .enumerate()
        .map(|(i, mob)| (mob.id.to_string(), i))
        .collect();
    let by_name = mobs.iter()
        .enumerate()
        .map(|(i, mob)| (slugify(&mob.name), i))
        .collect();

    MobIndex {
        mobs,
        by_id,
        by_name,
    }
}

lazy_static! {
    pub static ref MOBS: MobIndex = load_mobs();
}

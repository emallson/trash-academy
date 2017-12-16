use mobs::*;
use serde_yaml::from_reader;
use std::fs::File;
use askama::Template;
use std::collections::HashMap;

#[derive(Template, Default, Clone)]
#[template(path = "main.html")]
pub struct MainTemplate {}

pub struct InstanceIndex<'a> {
    instances: Vec<InstanceTemplate<'a>>,
    by_id: HashMap<String, usize>,
    by_name: HashMap<String, usize>,
    by_abbrv: HashMap<String, usize>,
}

impl<'a> InstanceIndex<'a> {
    pub fn by_id(&self, id: &str) -> Option<&InstanceTemplate> {
        self.by_id.get(id).map(|&i| &self.instances[i])
    }

    pub fn by_name(&self, id: &str) -> Option<&InstanceTemplate> {
        self.by_name.get(id).map(|&i| &self.instances[i])
    }

    pub fn by_abbrv(&self, id: &str) -> Option<&InstanceTemplate> {
        self.by_abbrv.get(id).map(|&i| &self.instances[i])
    }
}

#[derive(Deserialize)]
struct RawInstance {
    id: usize,
    name: String,
    forces: usize,
    teeming_forces: usize,
    mobs: Vec<usize>,
    abbrvs: Vec<String>,
}

#[derive(Template, Clone)]
#[template(path = "instance.html")]
pub struct InstanceTemplate<'a> {
    id: usize,
    name: String,
    _parent: MainTemplate,
    forces: usize,
    teeming_forces: usize,
    mobs: Vec<&'a Mob>,
    abbrvs: Vec<String>,
}

pub fn load_instances() -> InstanceIndex<'static> {
    let instances: Vec<RawInstance> = from_reader(File::open("data/instances.yaml").unwrap())
        .unwrap();

    let tpls = instances
        .into_iter()
        .map(|RawInstance {
             name,
             forces,
             teeming_forces,
             mobs,
             abbrvs,
             id,
         }| {
            InstanceTemplate {
                _parent: MainTemplate::default(),
                id,
                name,
                forces,
                teeming_forces,
                abbrvs,
                mobs: mobs.iter()
                    .map(|i| MOBS.by_id(&format!("{}", i)).unwrap())
                    .collect(),
            }
        })
        .collect::<Vec<_>>();

    let by_id = tpls.iter()
        .enumerate()
        .map(|(i, tpl)| (tpl.id.to_string(), i))
        .collect();
    let by_name = tpls.iter()
        .enumerate()
        .map(|(i, tpl)| (tpl.name.clone(), i))
        .collect();
    let by_abbrv = tpls.iter()
        .enumerate()
        .flat_map(|(i, tpl)| tpl.abbrvs.iter().map(move |ab| (ab.clone(), i)))
        .collect();

    InstanceIndex {
        instances: tpls,
        by_id,
        by_name,
        by_abbrv,
    }
}

lazy_static! {
    pub static ref INSTS: InstanceIndex<'static> = load_instances();
}

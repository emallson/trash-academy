extern crate iron;
#[macro_use]
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate logger;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate separator;

extern crate askama;
#[macro_use]
extern crate askama_derive;

use iron::prelude::*;
use staticfile::Static;
use mount::Mount;
use router::Router;
use logger::Logger;
use std::path::Path;
use askama::Template;
use iron::headers::ContentType;
use iron::modifiers::Header;
use separator::Separatable;

#[derive(Template)]
#[template(path = "main.html")]
struct MainTemplate {
}

#[derive(Template)]
#[template(path = "mob.html")]
struct MobTemplate<'a, 'b> {
    name: &'a str,
    percent: f64,
    teeming_percent: f64,
    abilities: &'b [Ability<'b>],
    _parent: MainTemplate,
}

struct Ability<'a> {
    id: usize,
    name: &'a str,
    base_description: &'a str,
    /// Contains the mythic +0 amounts of damage/healing/whatever done by the ability
    amounts: &'a [usize],
    description: String,
}

impl<'a> Ability<'a> {
    fn new(id: usize, name: &'a str, base_description: &'a str, amounts: &'a [usize]) -> Self {
        let mut ab = Ability {
            id, name, base_description, amounts, description: "".to_owned()
        };
        ab.description = ab.describe();
        ab
    }

    fn describe(&self) -> String {
        const LEVELS: &[usize] = &[1, 10, 20];
        let mut owned = self.base_description.to_owned();
        for &amount in self.amounts {
            let (low, mid, high) = scale(amount, LEVELS);
            let tpl = AmountTemplate {
                    amount_base: low,
                    amount_mid: mid,
                    amount_high: high,
                }
                .render()
                .unwrap();

            owned = owned.replace("%d", &tpl);
        }

        owned
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

fn mob(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok,
                       Header(ContentType::html()),
                       MobTemplate {
                           name: "Tank Butcherer",
                           percent: 2.0,
                           abilities: &[Ability::new(0,
                                                  "Shadow Slash",
                                                  "Deals YUUUGE Shadow damage to all \
                                                  players in a frontal cone. Will not \
                                                          turn while casting.",
                                                          &[],
                                                          ),
                                                          Ability::new(0,
                                                                    "Penetrating Shot",
                                                                    "Deals %d physical damage in a line 45 \
                                                                    yards in front of the caster. Ignores \
                                                          armor.",
                                                          &[1_600_000],
                                                          )],
                           teeming_percent: 1.2,
                           _parent: MainTemplate {},
                       })))
}

fn resources() -> Mount {
    let mut mount = Mount::new();
    mount.mount("/css/", Static::new(Path::new("./css/public/")));
    mount
}

fn mobs() -> Router {
    router! {
        mob: get "/:name" => mob,
    }
}

fn root() -> Mount {
    let mut mount = Mount::new();
    mount.mount("/resources/", resources());
    mount.mount("/mobs/", mobs());
    mount
}

pub fn start() {
    env_logger::init().unwrap();
    let (log_before, log_after) = Logger::new(None);
    let mut chain = Chain::new(root());
    chain.link_before(log_before);
    chain.link_after(log_after);
    Iron::new(chain).http("localhost:9001").unwrap();
}

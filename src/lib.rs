extern crate iron;
#[macro_use]
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate logger;
#[macro_use]
extern crate log;
extern crate env_logger;

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
    description: &'a str,
}

fn mob(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok,
                       Header(ContentType::html()),
                       MobTemplate {
                           name: "Tank Butcherer",
                           percent: 2.0,
                           abilities: &[Ability {
                                            id: 0,
                                            name: "Shadow Slash",
                                            description: "Deals YUUUGE Shadow damage to all \
                                                          players in a frontal cone. Will not \
                                                          turn while casting.",
                                        },
                                        Ability {
                                            id: 0,
                                            name: "Penetrating Shot",
                                            description: "Kills the nearest Mage immediately \
                                                          after they hit Combustion.",
                                        }],
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

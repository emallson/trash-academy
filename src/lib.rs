#[macro_use]
extern crate lazy_static;
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
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate slug;
mod instances;
mod mobs;

use iron::prelude::*;
use staticfile::Static;
use mount::Mount;
use router::Router;
use logger::Logger;
use std::path::Path;
use askama::Template;
use iron::headers::ContentType;
use iron::modifiers::Header;
use mobs::*;
use instances::*;

fn instance(req: &mut Request) -> IronResult<Response> {
    let router = req.extensions.get::<Router>().unwrap();
    let instance = if let Some(id) = router.find("id") {
        INSTS.by_id(id)
    } else if let Some(name) = router.find("name") {
        INSTS.by_name(name)
    } else if let Some(abbrv) = router.find("abbrv") {
        INSTS.by_abbrv(abbrv)
    } else {
        unreachable!()
    };

    if let Some(instance) = instance {
        Ok(Response::with((
            iron::status::Ok,
            Header(ContentType::html()),
            instance.clone(),
        )))
    } else {
        Ok(Response::with((iron::status::NotFound)))
    }
}

fn mob(req: &mut Request) -> IronResult<Response> {
    let router = req.extensions.get::<Router>().unwrap();
    let mob = if let Some(id) = router.find("id") {
        MOBS.by_id(id)
    } else if let Some(name) = router.find("name") {
        MOBS.by_name(name)
    } else {
        unreachable!()
    };

    if let Some(mob) = mob {
        Ok(Response::with((
            iron::status::Ok,
            Header(ContentType::html()),
            MobTemplate::new(mob),
        )))
    } else {
        Ok(Response::with((iron::status::NotFound)))
    }
}

fn resources() -> Mount {
    let mut mount = Mount::new();
    mount.mount("/css/", Static::new(Path::new("./css/public/")));
    mount
}

fn mobs() -> Router {
    router! {
        mob_id: get "/by-id/:id" => mob,
        mob_name: get "/:name" => mob,
    }
}

fn insts() -> Router {
    router! {
        id: get "/by-id/:id" => instance,
        name: get "/by-name/:name" => instance,
        abbrv: get "/:abbrv" => instance,
    }
}

fn root() -> Mount {
    let mut mount = Mount::new();
    mount.mount("/resources/", resources());
    mount.mount("/mobs/", mobs());
    mount.mount("/instances/", insts());
    mount
}

pub fn start() {
    use std::env::args;
    env_logger::init().unwrap();
    let (log_before, log_after) = Logger::new(None);
    let mut chain = Chain::new(root());
    chain.link_before(log_before);
    chain.link_after(log_after);
    Iron::new(chain).http(args().nth(1).unwrap()).unwrap();
}

extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

use std;
use iron::{Request, Response, status, IronResult};
use iron::prelude::*;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use self::mount::Mount;
use self::staticfile::Static;

use templating::{make_site, Section};

pub fn create_chain() -> Chain {

    let router = router!(root: get "/" => handle_root,
                         contact: get "/contact" => handle_contact,
                         quiz: get "/quiz" => handle_quiz);

    let mut mount = Mount::new();
    mount.mount("/", router).mount("/res/public/", Static::new(Path::new("res/public/")));

    Chain::new(mount)
}

fn handle_root(_: &mut Request) -> IronResult<Response> {
    let site = get_site("index.html");
    let site_template = make_site(Section::Home, &site);
    Ok(Response::with((site_template,
                       status::Ok)))
}

fn handle_contact(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Ferner Fanclub")))
}

fn handle_quiz(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Quizbois")))
}



fn get_site(path: &str) -> String {
    let mut whole_path = "res/templates/".to_string();
    whole_path.push_str(path);
    match File::open(&whole_path) {
        Err(_) => return get_site_not_found(path),
        Ok(mut val) => {
            let mut site = String::new();
            match val.read_to_string(&mut site) {
                Err(err) => return get_site_err(err),
                Ok(_) => return site,
            }
        }
    }
}

fn get_site_not_found(path: &str) -> String {
    let msg = format!("404, did not find site at {}", path);
    println!("{}", msg);
    msg
}

fn get_site_err<T: std::fmt::Display>(err: T) -> String {
    let msg = format!("Server error happened\n{}", err);
    println!("{}", msg);
    msg
}
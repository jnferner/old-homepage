extern crate iron;
extern crate iron_sessionstorage;
extern crate urlencoded;
extern crate handlebars;
extern crate handlebars_iron as hbs;

use self::iron::{Request, IronResult, Response, status};
use self::handlebars::to_json;
use presentation::helper::util::{get_formdata, to_ironresult};
use presentation::helper::templating::*;
use presentation::model::section::Section;
use business::crud::*;


pub fn get_admin(req: &mut Request) -> IronResult<Response> {
    let categories = get_categories()
        .unwrap()
        .into_iter()
        .map(|x| x.text)
        .collect::<Vec<String>>();
    let cat_json = btreemap! {
        "categories".to_string() => to_json(&categories),
    };
    let template = generate_site(req, "quiz/admin", cat_json, Some(&Section::Quiz));
    Ok(Response::with((template, status::Ok)))
}

pub fn post_admin(req: &mut Request) -> IronResult<Response> {
    let category = get_formdata(req, "category")?;
    let new_category = create_category(&category);
    to_ironresult(new_category)?;
    get_admin(req)
}

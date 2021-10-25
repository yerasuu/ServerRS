#![feature(proc_macro_hygiene, decl_macro)]

/// Rust packages for http requests
mod get;
mod post;
mod put;
mod delete;

#[macro_use]
extern crate rocket;
extern crate num_cpus;

use rocket::{Config, Request, config};
use rocket_dyn_templates::{Template, handlebars};

#[derive(serde::Serialize)]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
    items: Vec<&'static str>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

use self::handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};

fn wow_helper(
    h: &Helper<'_, '_>,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {   if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}

fn main() {
    let lcores_calc = ((num_cpus::get() as f32) * 0.75) as u16;
    let lcores = if lcores_calc > 1 { lcores_calc } else { 1 };
    let mut config = Config::from(config::Config::figment());
    rocket::custom(config)
        .mount("/", routes![get::index, get::hello, get::about])
        .register("/",catchers![not_found])
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("wow", Box::new(wow_helper));
        }))
        .launch();
}

use rocket_dyn_templates::Template;
use rocket::response::Redirect;
use crate::TemplateContext;

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to("/hello/Unknown")
}

#[get("/hello/<name>")]
pub fn hello(name: String) -> Template {
    Template::render("index", &TemplateContext {
        title: "Hello",
        name: Some(name),
        items: vec!["One", "Two", "Three"],
        parent: "layout",
    })
}

#[get("/about")]
pub fn about() -> Template {
    Template::render("about", &TemplateContext {
        title: "About",
        name: None,
        items: vec!["Four", "Five", "Six"],
        parent: "layout",
    })
}
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use tera::Tera;

static TEMPLATES_DIR: Dir = include_dir!("./templates");

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        for file in TEMPLATES_DIR.find("**/*.html").unwrap() {
            if let Some(template_file) = file.as_file() {
                if let Some(template_name) = file.path().file_name() {
                    match tera.add_raw_template(
                        template_name.to_str().unwrap(),
                        template_file.contents_utf8().unwrap(),
                    ) {
                        Ok(_) => (),
                        Err(e) => println!("Error adding template {} to compiled templates", e),
                    }
                }
            }
        }
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

use std::io::prelude::*;
use ansi_term::Colour::*;
use std::fs::*;
use generators::{Field, FieldType, Resource};

#[allow(non_snake_case)]
pub fn scaffold(resource: Resource, fields: Vec<Field>) {
    preamble(&resource);
    controller(&resource, &fields);
    model(&resource, &fields);
}

fn preamble(resource: &Resource) {
    create_dir("app").unwrap_or(());
    create_dir("app/controllers").unwrap_or(());
    create_dir("app/models").unwrap_or(());
    create_dir("app/helpers").unwrap_or(());
    create_dir(format!("app/controllers/{}", resource.plural)).unwrap_or(());
    create_dir(format!("app/controllers/{}/views", resource.plural)).unwrap_or(());
}

#[allow(non_snake_case)]
fn controller(resource: &Resource, fields: &Vec<Field>) {
    let mut controller = File::create(format!("app/controllers/{}/mod.rs", resource.plural))
        .expect("failed to create controller module");
    write!(controller, include_str!("scaffold/controller.rst"),
        resource = resource.name,
        resources = resource.plural,
        Resource = resource.constant
    ).unwrap();

    // TODO: Don't append controller if it already exists
    let mut controllers_mod = OpenOptions::new().append(true).open("app/controllers/mod.rs")
        .expect("failed to append controllers/mod.rs");
    write!(controllers_mod, "pub mod {};\n", resource.plural)
        .expect("failed to append controllers/mod.rs");

    // TODO: Detect app/layouts/mod.rs and update the module as appropriate
    let mut layouts_mod = OpenOptions::new().append(true).open("app/layouts.rs")
        .expect("failed to append app/layouts.rs");
    write!(layouts_mod, "\npub fn {r}(body: PreEscaped<String>) -> String {{\n    application(\"{r}\", body)\n}}\n", r= resource.plural)
        .expect("failed to append app/layouts.rs");

    let mut views = File::create(format!("app/controllers/{}/views/mod.rs", resource.plural))
        .expect("failed to create the view module");
    write!(views, include_str!("scaffold/views-mod.rst"),
        resource = resource.name
    ).expect("failed to write the view module");

    let mut views_index = File::create(format!("app/controllers/{}/views/index.rs", resource.plural))
        .expect("failed to create the index view");
    write!(views_index, include_str!("scaffold/views-index.rst"),
        resource = resource.name,
        resources = resource.plural,
        Resource = resource.constant
    ).expect("failed to write the view index");

    let show_fields = fields.iter().filter(|field| field.field_pub).fold(String::new(), |mut view, field| {
        view.push_str(&format!(include_str!("scaffold/views-show-field.rst"), resource = resource.name, field = field.field_name));
        view
    });

    let mut views_show = File::create(format!("app/controllers/{}/views/show.rs", resource.plural))
        .expect("failed to create the index view");
    write!(views_show, include_str!("scaffold/views-show.rst"),
        resource = resource.name,
        resources = resource.plural,
        Resource = resource.constant,
        fields = show_fields
    ).expect("failed to write the view index");

    let form_fields = fields.iter().fold(String::new(), |mut view, field| {
        view.push_str(&format!(include_str!("scaffold/views-form-field.rst"), resource = resource.name, field = field.field_name, field_type = field.field_type));
        view
    });

    let mut views_form = File::create(format!("app/controllers/{}/views/form.rs", resource.plural))
        .expect("failed to create the index view");
    write!(views_form, include_str!("scaffold/views-form.rst"),
        resource = resource.name,
        resources = resource.plural,
        Resource = resource.constant,
        fields = form_fields
    ).expect("failed to write the view index");
}

pub fn model(resource: &Resource, fields: &Vec<Field>) {
    let model_fields = fields.iter().fold(String::new(), |mut s, field| {
        s.push_str(&format!("\n    {}{}: {},",
                            if field.field_pub { "pub " } else { "" },
                            field.field_name, field.field_type
                           ));
        s
    });

    let model_fields_from_params = fields.iter().filter(|field| field.field_pub).fold(String::new(), |mut s, field| {
        s.push_str(&format!("\n    match params.find(&[\"{resource}\",\"{field}\"]).unwrap().clone() {{
        Value::String({field}) => {resource}.set_{field}({field}), _ => {{}}
    }}", resource = resource.name, field = field.field_name));
        s
    });

    let model_fields_default_values = fields.iter().fold(String::new(), |mut s, field| {
        s.push_str(&format!("\n            {field}: {field_type}::new(),", field = field.field_name, field_type = field.field_type));
        s
    });

    let model_fields_accessor_methods = fields.iter().fold(String::new(), |mut s, field| {
        s.push_str(&format!("\n    fn {field}(&self) -> &String {{ &self.title }}\n    fn set_title(&mut self, {field}: {field_type}) {{ self.{field} = {field} }}", field = field.field_name, field_type = field.field_type));
        s
    });

    let model_fields_accessor_interface = fields.iter().fold(String::new(), |mut s, field| {
        s.push_str(&format!("\n    fn {field}(&self) -> &String;\n    fn set_title(&mut self, {field}: {field_type});", field = field.field_name, field_type = field.field_type));
        s
    });
    let model_fields_validations: String = fields.iter().fold(String::new(), |mut s, field| {
        s.push_str(&format!("\n    if {resource}.{field}().is_empty() {{ errors.insert(\"{field}\", vec![\"can't be blank\"]); }}", resource = resource.name, field = field.field_name));
        s
    });

    let mut models = File::create(format!("app/models/{}.rs", resource.name))
        .expect("failed to create the index view");
    write!(models, include_str!("scaffold/model.rst"),
        resource = resource.name,
        resources = resource.plural,
        Resource = resource.constant,
        // TODO: Implement scaffold - model field generation
        fields = model_fields,
        fields_from_params = model_fields_from_params,
        fields_default_values = model_fields_default_values,
        fields_accessor_methods = model_fields_accessor_methods,
        fields_accessor_interface = model_fields_accessor_interface,
        fields_validations = model_fields_validations
    ).expect("failed to write the model");

    // TODO: Don't append model if it already exists
    let mut models_mod = OpenOptions::new().append(true).open("app/models/mod.rs")
        .expect("failed to append app/models/mod.rs");
    write!(models_mod, "pub mod {};\n", resource.name)
        .expect("failed to append models/mod.rs");

    let sql_fields: String = fields.iter().fold(String::new(), |mut s, field| {
        s.push_str(&field.sql_type());
        s
    });

    let migration_dir = format!("migrations/{}_{}_{}", resource.timestamp, "scaffold", resource.plural);
    create_dir(&migration_dir).unwrap();

    let mut migration_up = File::create(format!("{}/up.sql", migration_dir))
        .expect("failed to create the migration up");
    write!(migration_up, "CREATE TABLE {resources} (\n    id SERIAL PRIMARY KEY,{fields}\n)",
        resources = resource.plural,
        fields = sql_fields
    ).expect("failed to write the migration up");

    let mut migration_down = File::create(format!("{}/down.sql", migration_dir))
        .expect("failed to create the migration up");
    write!(migration_down, "DROP TABLE {resources};", resources = resource.plural,
    ).expect("failed to write the migration down");
    println!("migrations created apply them using:  {}\n", Green.bold().paint("diesel migration run"))

    // TODO: Implement scaffold - route generation (requires code parsing)
}


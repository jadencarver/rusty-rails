use std::io::prelude::*;
use std::fs::*;
use clap::Values;

struct Field {
    field_pub: bool,
    field_name: String,
    field_type: String
}

fn parse_attributes(attributes: Values) -> Vec<Field> {
    attributes.map( |attribute| {
        let mut split = attribute.split(':');
        let attr1 = split.next().unwrap_or("");
        let attr2 = split.next().unwrap_or("");
        let attr3 = split.next().unwrap_or("");
        if attr1 == "pub" {
            Field {
                field_pub: true,
                field_name: String::from(attr2),
                field_type: String::from(attr3)
            }
        } else {
            Field {
                field_pub: false,
                field_name: String::from(attr1),
                field_type: String::from(attr2)
            }
        }
    }).collect()
}

#[allow(non_snake_case)]
pub fn scaffold(resource: &str, attributes: Option<Values>) {
    let resource = resource;
    let resources = format!("{}s", resource);
    let Resource = format!("{}{}", &resource.to_uppercase()[0..1], &resource[1..]);

    create_dir("app").unwrap_or(());
    create_dir("app/controllers").unwrap_or(());
    create_dir("app/models").unwrap_or(());
    create_dir("app/helpers").unwrap_or(());
    create_dir(format!("app/controllers/{}", resources)).unwrap_or(());
    create_dir(format!("app/controllers/{}/views", resources)).unwrap_or(());

    // Parsing Field Arguments
    let fields: Vec<Field> = parse_attributes(attributes.unwrap());

    //-- Controllers
    let mut controller = File::create(format!("app/controllers/{}/mod.rs", resources))
        .expect("failed to create controller module");
    write!(controller, include_str!("rest-controller.rst"),
        resource = resource,
        resources = resources,
        Resource = Resource
    ).unwrap();

    //let mut controllers_mod = OpenOptions::new().append(true).open("app/controllers/mod.rs")
    //    .expect("failed to append controllers/mod.rs");
    //write!(controllers_mod, "pub mod {};\n", resources)
    //    .expect("failed to append controllers/mod.rs");

    // TODO: Detect app/layouts/mod.rs and update the module as appropriate
    //let mut layouts_mod = OpenOptions::new().append(true).open("app/layouts.rs")
    //    .expect("failed to append app/layouts.rs");
    //write!(layouts_mod, "\npub fn {}(body: PreEscaped<String>) -> String {{\n    application(\"{}\", body);\n}}\n", resources, resources)
    //    .expect("failed to append app/layouts.rs");

    let mut views = File::create(format!("app/controllers/{}/views/mod.rs", resources))
        .expect("failed to create the view module");
    write!(views, include_str!("rest-views-mod.rst"),
        resource = resource
    ).expect("failed to write the view module");

    let mut views_index = File::create(format!("app/controllers/{}/views/index.rs", resources))
        .expect("failed to create the index view");
    write!(views_index, include_str!("rest-views-index.rst"),
        resource = resource,
        resources = resources,
        Resource = Resource
    ).expect("failed to write the view index");

    let show_fields = fields.iter().filter(|field| field.field_pub).fold(String::new(), |mut view, field| {
        view.push_str(&format!(include_str!("rest-views-show-field.rst"), resource = resource, field = field.field_name));
        view
    });

    let mut views_show = File::create(format!("app/controllers/{}/views/show.rs", resources))
        .expect("failed to create the index view");
    write!(views_show, include_str!("rest-views-show.rst"),
        resource = resource,
        resources = resources,
        Resource = Resource,
        fields = show_fields
    ).expect("failed to write the view index");

    let form_fields = fields.iter().fold(String::new(), |mut view, field| {
        view.push_str(&format!(include_str!("rest-views-form-field.rst"), resource = resource, field = field.field_name, field_type = field.field_type));
        view
    });

    let mut views_form = File::create(format!("app/controllers/{}/views/form.rs", resources))
        .expect("failed to create the index view");
    write!(views_form, include_str!("rest-views-form.rst"),
        resource = resource,
        resources = resources,
        Resource = Resource,
        fields = form_fields
    ).expect("failed to write the view index");

    //-- MODELS --
    
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
    }}", resource = resource, field = field.field_name));
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
        s.push_str(&format!("\n    if {resource}.{field}().is_empty() {{ errors.insert(\"{field}\", vec![\"can't be blank\"]); }}", resource = resource, field = field.field_name));
        s
    });

    let mut models = File::create(format!("app/models/{}.rs", resource))
        .expect("failed to create the index view");
    write!(models, include_str!("model.rst"),
        resource = resource,
        resources = resources,
        Resource = Resource,
        // TODO: Implement scaffold - model field generation
        fields = model_fields,
        fields_from_params = model_fields_from_params,
        fields_default_values = model_fields_default_values,
        fields_accessor_methods = model_fields_accessor_methods,
        fields_accessor_interface = model_fields_accessor_interface,
        fields_validations = model_fields_validations
    ).expect("failed to write the model");

    //let mut models_mod = OpenOptions::new().append(true).open("app/models/mod.rs")
    //    .expect("failed to append app/models/mod.rs");
    //write!(models_mod, "pub mod {};\n", resource)
    //    .expect("failed to append models/mod.rs");

    // TODO: Implement scaffold - route generation (requires code parsing)
}

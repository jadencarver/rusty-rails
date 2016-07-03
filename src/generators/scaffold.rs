use std::io::prelude::*;
use std::fs::*;
use clap::Values;

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

    let fields: Vec<(_,_)> = attributes.unwrap().map( |attribute| {
        let mut split = attribute.split(':');
        (split.next().unwrap(), split.next().unwrap_or("String"))
    }).collect();

    let mut controller = File::create(format!("app/controllers/{}/mod.rs", resources))
        .expect("failed to create controller module");
    write!(controller, include_str!("rest-controller.rst"),
        resource = resource,
        resources = resources,
        Resource = Resource
    ).unwrap();

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

    let show_fields = fields.iter().fold(String::new(), |mut view, &(field, _)| {
        view.push_str(&format!(include_str!("rest-views-show-field.rst"), resource = resource, field = field));
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

    let form_fields = fields.iter().fold(String::new(), |mut view, &(field, field_type)| {
        view.push_str(&format!(include_str!("rest-views-form-field.rst"), resource = resource, field = field, field_type = field_type));
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

    //OpenOptions::new().read(true).write(true)
}

use generators::{Generator, Resource, Model, Controller, View};
use clap::{ArgMatches, Values};

pub struct Scaffold {
    resource: Resource,
//    model: Model,
//    controller: Controller,
//    views: Vec<View>
}

impl Scaffold {
    fn new(resource: Resource) -> Result<Box<Scaffold>, Vec<String>> {
        Ok(Box::new(Scaffold {
            resource: resource,
            //model: Model::new(args),
            //controller: Controller::new(controller),
            //views: vec![
            //    View::new("index"),
            //    View::new("show"),
            //    View::new("forms")
            //]
        }));
    }
}


impl Generator for Scaffold {
    fn generate(&self) {}
}

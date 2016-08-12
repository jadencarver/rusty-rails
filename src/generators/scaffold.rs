use generators::{Generator, GeneratorError, Resource, Model, Controller};

pub struct Scaffold {
    model: Box<Model>,
    controller: Box<Controller>,
    resource: Resource
}

impl Generator for Scaffold {
    fn new(resource: Resource) -> Result<Box<Scaffold>, GeneratorError> {
        Ok(Box::new(Scaffold {
            model: try!(Model::new(resource.clone())),
            controller: try!(Controller::new(resource.clone())),
            resource: resource
        }))
    }
    fn generate(self) {}
}

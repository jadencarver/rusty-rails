use generators::{Generator, GeneratorError, Resource, Migration};

pub struct Model {
    migration: Box<Migration>,
    resource: Resource
}

impl Generator for Model {
    fn new(resource: Resource) -> Result<Box<Model>, GeneratorError> {
        Ok(Box::new(Model {
            migration: try!(Migration::new(resource.clone())),
            resource: resource
        }))
    }
    fn generate(self) {}
}

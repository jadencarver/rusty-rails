use generators::{Generator, GeneratorError, Resource};

pub struct Migration {
    resource: Resource
}

impl Generator for Migration {
    fn new(resource: Resource) -> Result<Box<Migration>, GeneratorError> {
        Ok(Box::new(Migration {
            resource: resource
        }))
    }
    fn generate(self) {}
}

use generators::{Generator, GeneratorError, Resource, View};
use std::path::PathBuf;
use std::fs::*;

pub struct Controller {
    resource: Resource,
    path: PathBuf,
}

impl Generator for Controller {
    fn new(resource: Resource) -> Result<Box<Controller>, GeneratorError> {
        let path = PathBuf::from(format!("app/controllers/{}", resource.name.plural()));
        if path.exists() {
            return Err(GeneratorError {
                description: format!("{:?} already exists", path), cause: None
            });
        }
        Ok(Box::new(Controller {
            resource: resource,
            path: path
        }))
    }
    fn generate(self) {
        create_dir("app/controllers").unwrap_or(());
        create_dir(self.path).unwrap_or(());
    }
}

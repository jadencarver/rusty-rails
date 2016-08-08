mod scaffold;
mod controller;
mod model;
mod migration;
mod view;

pub use generators::scaffold::Scaffold;
pub use generators::controller::Controller;
pub use generators::model::Model;
pub use generators::migration::Migration;
pub use generators::view::View;

use clap::{ArgMatches, Values};

enum LiteralType {
    // concrete types:
    String(i32), Text(i32),
    Boolean, Integer, Decimal, Float, DateTime, Date,
    // abstract types:
    Symbol, Color, Image, Video, File, Url,
    Phone, Email, Password, Search
}

enum Access {
    Required, Optional, Private
}

struct Field {
    name: String,
    access: Access,
    literal: LiteralType
}

struct Resource {
    name: String,
    fields: Vec<Field>
}

trait Generator {
    fn generate(&self);
}

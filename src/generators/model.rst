use diesel::prelude::*;
use schema::{resources};
use std::fmt;

use std::collections::HashMap;
pub type Errors = Option<HashMap<&'static str, Vec<&'static str>>>;
use params::{{Map, Value}};

pub trait {Resource}Model {{
    fn update(&mut self, params: Map);
    fn is_valid(&mut self) -> Result<bool, Errors>;{fields_accessor_interface}
}}

fn validate<{Resource}: {Resource}Model>({resource}: &{Resource}) -> Result<bool, Errors> {{
    let mut errors = HashMap::new();
{fields_validations}

    if errors.is_empty() {{
        Ok(true)
    }} else {{
        Err(Some(errors))
    }}
}}

fn update<{Resource}: {Resource}Model>({resource}: &mut {Resource}, params: Map) {{{fields_from_params}
}}

#[derive(Queryable)]
#[insertable_into({resources})]
#[changeset_for({resources})]
pub struct {Resource} {{
    pub id: i32, {fields}
}}

impl {Resource} {{
    pub fn new() -> New{Resource} {{
        New{Resource} {{{fields_default_values}
        }}
    }}
}}

impl {Resource}Model for {Resource} {{
    fn update(&mut self, params: Map) {{ update(self, params) }}
    fn is_valid(&mut self) -> Result<bool, Errors> {{ validate(self) }}{fields_accessor_methods}
}}

impl fmt::Display for {Resource} {{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {{
        fmt.write_str(&format!("{Resource} #{{}}", self.id))
    }}
}}

#[insertable_into({resources})]
pub struct New{Resource} {{{fields}
}}

impl {Resource}Model for New{Resource} {{
    fn update(&mut self, params: Map) {{ update(self, params) }}
    fn is_valid(&mut self) -> Result<bool, Errors> {{ validate(self) }}{fields_accessor_methods}
}}


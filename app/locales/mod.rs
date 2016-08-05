enum Language {
    En,
    Es,

}

struct Locale {
    language: str,
    extlang Option(str)
    script: Option(str),
    region: Option(str),
    variant: Option(str),
    extension: Option(str),
    private: Option(str)
}

impl Locale {
    pub fn new(tags: Vec<&str>) -> Locale {
    }

    pub fn translate(&self, key: &str) -> &str {
    }

    pub fn t(&self, key: &str) -> &str {
    }
}

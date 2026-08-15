#[derive(Debug, Default)]
pub struct Doi(String);

impl Doi {
    pub fn new(doi_value: String) -> Self {
        Self{
            0: doi_value,
        }
    }
}
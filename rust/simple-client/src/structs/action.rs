pub trait Action {
    fn to_string(&self, index: &i8) -> String; 
}

pub struct Advance {
    pub distance: i8,
}

impl Action for Advance {
    fn to_string(&self, index: &i8) -> String {
        format!("<advance order=\"{}\" distance=\"{}\"/>", index, self.distance)
    }
}
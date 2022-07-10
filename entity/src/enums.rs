use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Display, PartialEq, EnumString, AsRefStr)]
pub enum Role {
    Admin,
    Writer,
    Editor,
}

#[derive(Display, PartialEq, EnumString, AsRefStr)]
pub enum Status {
    Published,
    Disabled,
    Draft,
}

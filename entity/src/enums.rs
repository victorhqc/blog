use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Display, PartialEq, EnumString, AsRefStr)]
pub enum Role {
    Admin,
    Writer,
    Editor,
}

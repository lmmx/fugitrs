use smart_default::SmartDefault;

#[derive(SmartDefault, Debug)]
pub struct Config {
    #[default = 1]
    pub param1: i32,
    pub param2: String,
    #[default = true]
    pub param3: bool,
}

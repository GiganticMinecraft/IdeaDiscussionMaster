#[derive(Clone, PartialEq)]
pub enum SlashCommandChoice {
    String(String),
    Int(i32),
    Number(f64),
}

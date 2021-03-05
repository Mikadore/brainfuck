/// Enum for the brainfuck commands
/// | Name | Source |  Full name |
/// |------|--------|------------|
/// | Lpe  | ]      | Loop end   |
/// | Lps  | [      | Loop start |
/// | Incr | +      | Increment  |
/// | Decr | -      | Decrement  |
/// | Datg | ,      | Data get   |
/// | Datp | .      | Data put   |
/// | Mvr  | >      | Move right |
/// | Mvl  | <      | Move left  |
pub enum Commands {
    Lpe,
    Lps,
    Incr,
    Decr,
    Datg,
    Datp,
    Mvr,
    Mvl
}
impl Commands {
    pub fn val(&self) -> isize {
        match self {
            Commands::Incr =>  1,
            Commands::Decr => -1,
            Commands::Mvr  =>  1,
            Commands::Mvl  => -1,
            _ => 0
        }
    }
}
impl std::fmt::Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Commands::*;
        write!(f, "{}", match *self {
            Lpe         => "Lpe",
            Lps         => "Lps",
            Incr        => "Incr", 
            Decr        => "Decr",
            Datg        => "Datg",
            Datp        => "Datp",
            Mvr         => "Mvr", 
            Mvl         => "Mvl",
        })   
    }
}
pub mod loops;
pub mod ast;
pub mod interpreter;
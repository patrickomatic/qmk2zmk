#[derive(Debug, Clone)]
pub struct Keymap {
    pub keyboard: Option<String>,
    pub layout: Option<String>,
    pub layers: Vec<Layer>,
    pub macros: Vec<MacroDef>,
    pub tri_layer: Option<TriLayer>,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub index: usize,
    pub keys: Vec<Key>,
}

#[derive(Debug, Clone)]
pub enum Key {
    Kp(String),
    Mo(usize),
    Lt(usize, String),
    Mt(String, String),
    Tog(usize),
    Sk(String),
    Sl(usize),
    To(usize),
    Df(usize),
    Mmv(String),
    Mkp(String),
    Msc(String),
    Trans,
    None,
    CapsWord,
    Bootloader,
    SysReset,
    RgbUg(String),
    Macro(String),
    Unknown(String),
}

#[derive(Debug, Clone)]
pub struct MacroDef {
    pub name: String,
    pub steps: Vec<MacroStep>,
}

#[derive(Debug, Clone)]
pub enum MacroStep {
    Tap(String),
    Wait(u32),
}

#[derive(Debug, Clone)]
pub struct TriLayer {
    pub lower: usize,
    pub upper: usize,
    pub tri: usize,
}

use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dims {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum ElemStyle {
    Left,
    Right,
    //    Bar,
    //    Line,
    #[default]
    None,
    //    Tick,
    //    Spacer,
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum Colour {
    Black,
    Blue,
    BlueViolet,
    CadetBlue,
    DarkGrey,
    DarkOrange,
    DarkSalmon,
    DarkGoldenRod,
    DarkKhaki,
    DarkOliveGreen,
    DarkOrchid,
    FireBrick,
    GoldenRod,
    LemonChiffon,
    #[default]
    LightGrey,
    LightGreen,
    LightBlue,
    LightCoral,
    MediumVioletRed,
    MediumSlateBlue,
    MediumAquamarine,
    Orange,
    OrangeRed,
    OliveDrab,
    Orchid,
    Plum,
    Purple,
    RebeccaPurple,
    Red,
    RoyalBlue,
    Salmon,
    SlateBlue,
    SteelBlue,
    SeaGreen,
    Turquoise,
    Tomato,
    Violet,
    White,
    Yellow,
    YellowGreen,
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}

#[derive(Clone, Debug)]
pub struct ElementData {
    pub style: ElemStyle,
    pub label: Option<String>,
    pub start: f32,
    pub end: f32,
    pub scale: f32,
    pub colour: Colour,
}

#[derive(Clone, Debug)]
pub struct TrackData {
    pub height: f32,
    pub elements: Vec<ElementData>,
}

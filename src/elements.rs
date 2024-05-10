use std::fmt;

use serde::{Deserialize, Serialize};
//use serde_json::Result;

#[derive(Copy, Clone, Debug)]
pub struct Dims {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ElemStyle {
    Left,
    Right,
    Bar,
    Line,
    #[default]
    Rect,
    Tick,
    Spacer,
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Colour {
    Black,
    White,
    #[default]
    Lightgrey,
    Lightblue,
    Salmon,
    Orange,
    Turquoise,
    Yellowgreen,
    Plum,
    Red,
    Darkgrey,
    Mediumaquamarine,
    Blue,
    Firebrick,
    Slateblue,
    Steelblue,
}

/*
#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct FigureData {
    tracks: Vec<TrackData>,
}
*/

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct ElementData {
    #[serde(default)]
    pub style: ElemStyle,
    #[serde(default)]
    pub label: Option<String>,
    pub start: f32,
    pub end: f32,
    #[serde(default = "default_scale")]
    pub scale: f32,
    #[serde(default)]
    pub colour: Colour,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackData {
    #[serde(default = "default_height")]
    pub height: f32,
    pub elements: Vec<ElementData>,
}

fn default_height() -> f32 {
    20.0
}
fn default_scale() -> f32 {
    1.0
}

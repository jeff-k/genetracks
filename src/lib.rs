use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fmt;
use std::rc::Rc;
use yew::{html, Html};

#[derive(Serialize, Deserialize)]
pub struct Figure {
    width: usize,
    height: usize,
    tracks: Vec<Track>,
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub height: usize,
    pub elems: Vec<Elem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Elem {
    #[serde(default)]
    pub style: Style,
    pub label: Rc<str>,
    pub start: usize,
    pub length: usize,
    pub colour: Rc<str>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Style {
    Rect,
    Line,
    Bar,
    Left,
    Right,
}

impl Default for Style {
    fn default() -> Style {
        Style::Rect
    }
}

impl Elem {
    pub fn draw(&self, scale: f32, height: f32) -> Html {
        let end: f32 = self.length as f32 / scale;
        let midpoint = (self.length as f32 / 2.0) / scale;
        let start: f32 = (self.start as f32) / scale;
        match self.style {
            Style::Left => html! {

                <g transform={format!("translate({} 0)", start)}>
                <path d={format!("M0,0 L{},0 L{},{} L0,{} L{},{} L0,0", end, end, height, height, -10.0, height / 2.0)} fill={self.colour.to_string()} stroke={self.colour.to_string()} stroke-opacity="0.0" />
                <text x={format!("{}", midpoint)} y={ format!("{}", height / 2.0) } font-size="12" font-family="monospace" text-anchor="middle" dominant-baseline="middle">{ self.label.to_string() }</text>
                </g>


            },

            Style::Right => html! {

                <g transform={format!("translate({} 0)", start)}>
                <path d={format!("M0,0 L{},0 L{},{} L{},{} L0,{} L0,0", end, end + 10.0, height / 2.0, end, height, height)} fill={self.colour.to_string()} stroke={self.colour.to_string()} stroke-opacity="0.0" />
                <text x={format!("{}", midpoint)} y={format!("{}", height / 2.0)} font-size="12" font-family="monospace" text-anchor="middle" dominant-baseline="middle">{ self.label.to_string() }</text>
                </g>


            },
            Style::Line => html! {
                <g transform={format!("translate({} 0)", start)}>
                <path d={format!("M0.0,{} L{},{}", height / 2.0, end, height / 2.0)} stroke={self.colour.to_string()} />
                </g>
            },
            Style::Bar => html! {
                <g transform={format!("translate({} 0)", start)}>
                <path d={format!("M0.0,0.0 L0.0,{}", height)} stroke={self.colour.to_string()} />
                <path d={format!("M0.0,{} L{},{}", height / 2.0, end, height / 2.0)} stroke={self.colour.to_string()} />
                <path d={format!("M{},0.0 L{},{}", end, end, height)} stroke={self.colour.to_string()} />

                </g>
            },
            _ => html! {

                <g transform={format!("translate({} 0)", start)}>
                        <rect x="0" y="0" width={format!("{}", end)} height={format!("{}", height)} fill={self.colour.to_string()} stroke={self.colour.to_string()} stroke-opacity="0.0"/>

                <text x={format!("{}", midpoint)} y={format!("{}", height / 2.0)} font-size="12" font-family="monospace" text-anchor="middle" dominant-baseline="middle">{ self.label.to_string() }</text>
                </g>
            },
        }
    }
}

impl Track {
    fn draw(&self, row: usize, scale: f32) -> Html {
        html! { <g transform={format!("translate(0 {})", row as f32)}>
            { self.elems.iter().map(|e| e.draw(scale, self.height as f32)).collect::<Html>() }
        </g> }
    }
}

impl Figure {
    pub fn new(tracks: Vec<Track>) -> Self {
        Self {
            width: 1000,
            height: 200,
            tracks,
        }
    }

    pub fn push_interval(&mut self, s: usize, e: usize) {
        self.tracks.push(Track {
            height: 16,
            elems: vec![Elem {
                style: Style::Bar,
                label: Rc::from(""),
                start: s,
                length: e - s,
                colour: Rc::from("grey"),
            }],
        });
    }
    pub fn to_svg(&self) -> Html {
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;
        let padding = 3;

        for track in &self.tracks {
            height += track.height as f32 + padding as f32;
            for elem in &track.elems {
                let end = elem.start as f32 + elem.length as f32;
                if end >= width {
                    width = end;
                }
            }
        }
        let scale: f32 = width / self.width as f32;
        let mut row_y: usize = 0;
        html! {
        <svg width={ format!("{}", self.width) } height={format!("{}", height)} viewBox={format!("0.0 0.0 {} {}", self.width, height)} preserveAspectRatio={ "none" }>
        <defs>
        </defs>
        { self.tracks.iter().map(|track| {
            let svg_node = track.draw(row_y, scale);
            row_y += track.height + padding;
            svg_node
            }).collect::<Html>() }
        </svg>
                }
    }

    pub fn from_string(s: String) -> Result<Self> {
        serde_json::from_str(&s)
    }
}

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

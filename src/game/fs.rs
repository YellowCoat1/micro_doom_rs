use crate::game::lines::LineSegment;
use super::vecs::Vec2;

pub fn segs_from_file(s: &str) -> Vec<(Vec2, Vec2)> {
    let mut line_segments: Vec<(Vec2, Vec2)> = vec![];
    let contents = std::fs::read_to_string(s).expect("Failed to read file :(");
    for line in contents.lines() {
        let coords: Vec<f32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse coordinates :("))
            .collect();
        if coords.len() == 4 {
            line_segments.push(((coords[0], coords[1]).into(), (coords[2], coords[3]).into()));
        }
    }

    line_segments
}

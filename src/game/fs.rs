use crate::game::lines::LineSegment;
use super::vecs::Vec2;

pub fn segs_from_file() -> Vec<(Vec2, Vec2)> {
    let mut line_segments: Vec<(Vec2, Vec2)> = vec![];
    let contents = std::fs::read_to_string("map01.txt").expect("Failed to read file :(");
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


// exactly what it says on the tin.
// Reads line segments from a file.
/*fn read_line_segments_from_file(filename: &str) -> Vec<LineSegment> {
    let mut line_segments = vec![];
    let contents = std::fs::read_to_string(filename).expect("Failed to read file");
    for line in contents.lines() {
        let coords: Vec<f64> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse coordinate"))
            .collect();
        if coords.len() == 4 {
            line_segments.push(LineSegment::new(
                coords[0], coords[1], coords[2], coords[3],
            ));
        }
    }
    line_segments
}*/
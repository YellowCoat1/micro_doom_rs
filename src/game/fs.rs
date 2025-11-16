use super::vecs::Vec2;

// returns camera position and line segments from a file
pub fn segs_from_file() -> (Vec<(Vec2, Vec2)>, Vec2) {
    let mut args  = std::env::args().skip(1);
    let s = match args.next() {
        Some(arg) => arg,
        None => "map01.txt".to_string(),
    };
    let mut line_segments: Vec<(Vec2, Vec2)> = vec![];
    let contents = std::fs::read_to_string(s).expect("Failed to read file :(");
    let lines = contents.lines();

    let cam_line = lines.clone().next().expect("File is empty :(");
    let cam_coords: Vec<_> = cam_line
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse camera coordinates :("))
        .collect();

    assert!(cam_coords.len() >= 2, "Camera coordinates (start of file) should have 2 values :(");

    let camera_pos = if cam_coords.len() == 2 {
        (cam_coords[0], cam_coords[1]).into()
    } else {
        panic!("Camera coordinates (start of file) should have 2 values :(");
    };

    for line in contents.lines() {
        let coords: Vec<f32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse coordinates :("))
            .collect();
        if coords.len() == 4 {
            line_segments.push(((coords[0], coords[1]).into(), (coords[2], coords[3]).into()));
        }
    }

    (line_segments, camera_pos)
}

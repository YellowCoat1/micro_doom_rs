use pixels::Pixels;

use super::Color;

pub struct PixelDrawer<'a> {
    pixels: &'a mut Pixels,
}

impl <'a> PixelDrawer<'a> {
    pub fn new(pixels: &'a mut Pixels) -> Self {
        PixelDrawer { pixels }
    }

    pub fn dims(&self) -> Option<(i32, i32)>{
        let e = self.pixels.context().texture_extent;
        Some((e.width as i32, e.height as i32))
    }
    pub fn clear(&mut self, color: Color) {
        let (width, height) = match self.dims() {
            Some(dims) => dims,
            None => return,
        };
        let buffer = self.pixels.frame_mut();
        for y in 0..height {
            for x in 0..width {
                let offset = (y*width + x)*4;
                buffer[offset as usize] = color.0;
                buffer[(offset+1) as usize] = color.1;
                buffer[(offset+2) as usize] = color.2;
                buffer[(offset+3) as usize] = 255;
            }
        }
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        let (width, height) = match self.dims() {
            Some(dims) => dims,
            None => return,
        };
        let buffer = self.pixels.frame_mut();

        let dx = (x2 - x1).abs() * 2;
        let dy = (y2 - y1).abs() * 2;

        let stepx = if x1 < x2 { 1 } else { -1 };
        let stepy = if y1 < y2 { 1 } else { -1 };
        
        let mut point = |x, y| draw_point(buffer, width, height, x, y, color);
        
        point(x1, y1);

        if dx > dy {
            let mut fraction = dy - (dx/2);
            let (mut x, mut y) = (x1, y1);
            while x != x2 {
                x += stepx;
                if fraction >= 0 {
                    y += stepy;
                    fraction -= dx;
                }
                fraction += dy;
                point(x, y);
            }
        } else {
            let mut fraction = dx - (dy/2);
            let (mut x, mut y) = (x1, y1);
            while y != y2 {
                if fraction >= 0 {
                    x += stepx;
                    fraction -= dy;
                }
                y += stepy;
                fraction += dx;
                point(x, y);
            }
        }
    }

    pub fn triangle(&mut self, p1: (i32, i32), p2: (i32, i32), p3: (i32, i32), color: Color) {
        // sort points by y
        let mut v = [(p1.0, p1.1), (p2.0, p2.1), (p3.0, p3.1)];
        v.sort_by(|a, b| a.1.cmp(&b.1));
        let (p1, p2, p3) = (v[0], v[1], v[2]);

        if p2.1 == p3.1 {
            // flat bottom
            // ensure left/right order for bottom x's
            let (b_left, b_right) = if p2.0 <= p3.0 { (p2.0, p3.0) } else { (p3.0, p2.0) };
            self.wide_bottom_triangle(p1, p2.1, b_left, b_right, color);
        } else if p1.1 == p2.1 {
            // flat top
            // ensure left/right order for top x's
            let (t_left, t_right) = if p1.0 <= p2.0 { (p1.0, p2.0) } else { (p2.0, p1.0) };
            self.wide_top_triangle(p1.1, t_left, t_right, p3, color);
        } else {
            // general case - split the triangle in a flat top and flat bottom
            let midpoint = x_at_y(p1.0, p1.1, p3.0, p3.1, p2.1);

            let (x_left, x_right) = if midpoint < p2.0 {
                (midpoint, p2.0)
            } else {
                (p2.0, midpoint)
            };

            self.wide_bottom_triangle(p1, p2.1, x_left, x_right, color);
            self.wide_top_triangle(p2.1, x_left, x_right, p3, color);
        }
    }

    pub fn wide_bottom_triangle(&mut self, p1: (i32, i32), p23y: i32, p2x: i32, p3x: i32, color: Color) {
        assert!(p1.1 < p23y);
        let (width, height) = match self.dims() {
            Some(dims) => dims,
            None => return,
        };
        let buffer = self.pixels.frame_mut();

        for y in p1.1..=p23y {
            let mut start_x = x_at_y(p1.0, p1.1, p2x, p23y, y);
            let mut end_x   = x_at_y(p1.0, p1.1, p3x, p23y, y);

            if start_x > end_x {
                std::mem::swap(&mut start_x, &mut end_x);
            }

            for x in start_x..=end_x {
                draw_point(buffer, width, height, x, y, color);
            }
        }
    }

    pub fn wide_top_triangle(&mut self, p12y: i32, p1x: i32, p2x: i32, p3: (i32, i32), color: Color) {
        assert!(p12y < p3.1);
        let (width, height) = match self.dims() {
            Some(dims) => dims,
            None => return,
        };
        let buffer = self.pixels.frame_mut();

        for y in p12y..=p3.1 {
            let mut start_x = x_at_y(p1x, p12y, p3.0, p3.1, y);
            let mut end_x   = x_at_y(p2x, p12y, p3.0, p3.1, y);

            if start_x > end_x {
                std::mem::swap(&mut start_x, &mut end_x);
            }

            for x in start_x..=end_x {
                draw_point(buffer, width, height, x, y, color);
            }
        }
    }

    pub fn draw_quad(&mut self, p1: (i32, i32), p2: (i32, i32), p3: (i32, i32), p4: (i32, i32), color: Color) {

        // compute centroid
        let cx = (p1.0 + p2.0 + p3.0 + p4.0) as f32 / 4.0;
        let cy = (p1.1 + p2.1 + p3.1 + p4.1) as f32 / 4.0;

        let mut ordered = [p1, p2, p3, p4];
        let angle = |p: (i32, i32)| (p.1 as f32 - cy).atan2(p.0 as f32 - cx);
        ordered.sort_by(|a, b| angle(*a).partial_cmp(&angle(*b)).unwrap());
        let (p1, p2, p3, p4) = (ordered[0], ordered[1], ordered[2], ordered[3]);
        self.triangle(p1, p2, p3, color);
        self.triangle(p1, p3, p4, color);
    }

    pub fn draw_poly(&mut self, points: &[(i32, i32)], color: Color) {
        //make a lil box around it
        let (xs, ys): (Vec<i32>, Vec<i32>) = points.iter().copied().unzip();

        let (mut x_max, mut x_min) = (xs[0], xs[0]);
        for x in xs.iter().skip(1) {
            if *x < x_min {
                x_min = *x
            } else if *x > x_max {
                x_max = *x
            }
        }

        let (mut y_max, mut y_min) = (ys[0], ys[0]);
        for y in ys.iter().skip(1) {
            if *y < y_min {
                y_min = *y
            } else if *y > y_max {
                y_max = *y
            } 
        }
       
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                if in_poly((x, y), points) {
                    self.draw_point(x, y, color);
                }
            }
        }
    }


    pub fn draw_point(&mut self, x: i32, y: i32, color: Color) {
        let (width, height) = match self.dims() {
            Some(dims) => dims,
            None => return,
        };
        let buffer = self.pixels.frame_mut();
        draw_point(buffer, width, height, x, y, color);
    }


}

fn draw_point(buffer: &mut [u8], width: i32, height: i32, x: i32, y: i32, color: Color) {
    if x<0 || y<0 || x>=width || y>=height {
        return;
    }
    let offset = (y*width + x)*4;
    buffer[offset as usize] = color.0;
    buffer[(offset+1) as usize] = color.1;
    buffer[(offset+2) as usize] = color.2;
    buffer[(offset+3) as usize] = 255;
}

fn x_at_y(x1: i32, y1: i32, x2: i32, y2: i32, y: i32) -> i32 {
    if y2 == y1 {
        return x1; // horizontal line: any x in the range is valid
    }

    let dy = y - y1;
    let total_dy = y2 - y1;
    let dx = x2 - x1;

    // integer interpolation: x = x1 + dx * (y - y1) / (y2 - y1)
    // if you prefer rounding instead of truncation, use the commented line below.
    x1 + (dx * dy) / total_dy
    // x1 + (dx * dy + total_dy.signum() * (total_dy.abs()/2)) / total_dy
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
    On,
}

pub fn side(a: (i32, i32), b: (i32, i32), p: (i32, i32)) -> Side {
    let t1 = (b.0 - a.0)*(p.1 - a.1);
    let t2 = (b.1 - a.1)*(p.0 - a.0);
    
    if t1 > t2 {
        Side::Left
    } else if t1 < t2 {
        Side::Right
    } else {
        Side::On
    }
}

fn center(points: &[(i32, i32)]) -> (f32, f32) {
    let (xs, ys): (Vec<i32>, Vec<i32>) = points.iter().cloned().unzip();
    let x_avg = xs.iter().sum::<i32>() as f32 / xs.len() as f32;
    let y_avg = ys.iter().sum::<i32>() as f32 / ys.len() as f32;

    (x_avg, y_avg)
}

pub fn in_poly(point: (i32, i32), points: &[(i32, i32)]) -> bool {
    let line_segs = points.windows(2).map(|v| (v[0], v[1]));
    let line_segs = line_segs.chain(std::iter::once((points[points.len()-1], points[0])));
    let center = center(points);
    let center = (center.0 as i32, center.1 as i32);
    let base_sides = line_segs.clone()
        .map(|v| side(v.0, v.1, center));

    for ((p1, p2), reg_side) in line_segs.zip(base_sides) {
        let side = side(p1, p2, point);
        if side != reg_side && side != Side::On {
            return false
        }
    }
    true
}

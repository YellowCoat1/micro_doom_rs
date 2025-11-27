use super::lines::{LineSegment, Order, split_line};
use super::vecs::Vec2;

pub struct BSPNode {
    partition: LineSegment,
    front: Option<Box<BSPNode>>,
    back: Option<Box<BSPNode>>,
}

const EPSILON: f32 = 1e-5;
fn almost_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

fn normalize_partition(mut partition: LineSegment) -> LineSegment {
    if partition.start.x > partition.end.x
        || (partition.start.x == partition.end.x && partition.start.y > partition.end.y)
    {
        std::mem::swap(&mut partition.start, &mut partition.end);
    }
    partition
}

fn bsp_point_side_of_line(line: &LineSegment, point: &Vec2) -> Order {
    // cross = (line ->) x (line->point)
    let cross = (line.end.x - line.start.x) * (point.y - line.start.y)
        - (line.end.y - line.start.y) * (point.x - line.start.x);
    if cross > EPSILON {
        Order::Left
    } else if cross < -EPSILON {
        Order::Right
    } else {
        Order::On
    }
}

fn is_degenerate(seg: &LineSegment) -> bool {
    almost_equal(seg.start.x, seg.end.x) && almost_equal(seg.start.y, seg.end.y)
}
fn classify_segment(
    partition: LineSegment,
    segment: LineSegment,
) -> (Option<LineSegment>, Option<LineSegment>) {
    let mut start_side = bsp_point_side_of_line(&partition, &segment.start);
    let mut end_side = bsp_point_side_of_line(&partition, &segment.end);

    // If one endpoint is On, treat it as being on the same side as the other endpoint.
    // This avoids unnecessary splits into zero-length pieces.
    if start_side == Order::On && end_side != Order::On {
        start_side = end_side;
    } else if end_side == Order::On && start_side != Order::On {
        end_side = start_side;
    }

    match (start_side, end_side) {
        (Order::On, Order::On) => {
            // perfectly colinear: keep it on the front side (or choose whichever side you prefer)
            (Some(segment), None)
        }
        (Order::Left, Order::Left) => (Some(segment), None),
        (Order::Right, Order::Right) => (None, Some(segment)),
        // different sides -> actually split
        _ => {
            if let Some((left, right)) = split_line(&partition, &segment) {
                // Guard against degenerate zero-length pieces produced by split_line
                let left_opt = if is_degenerate(&left) {
                    None
                } else {
                    Some(left)
                };
                let right_opt = if is_degenerate(&right) {
                    None
                } else {
                    Some(right)
                };
                (left_opt, right_opt)
            } else {
                // split_line couldn't split for some reason — be conservative
                (Some(segment), None)
            }
        }
    }
}

impl BSPNode {
    pub fn new(partitions: Vec<LineSegment>) -> Self {
        if partitions.is_empty() {
            panic!("Cannot create BSPNode with no partitions");
        }
        let partition = normalize_partition(partitions[0]);
        let mut front_partitions: Vec<LineSegment> = Vec::new();
        let mut back_partitions: Vec<LineSegment> = Vec::new();

        for part in partitions.iter().skip(1) {
            let normalized_part = normalize_partition(*part);

            let (front, back) = classify_segment(partition, normalized_part);
            if let Some(f) = front {
                front_partitions.push(f);
            }
            if let Some(b) = back {
                back_partitions.push(b);
            }
        }
        BSPNode {
            partition,
            front: if front_partitions.is_empty() {
                None
            } else {
                Some(Box::new(BSPNode::new(front_partitions)))
            },
            back: if back_partitions.is_empty() {
                None
            } else {
                Some(Box::new(BSPNode::new(back_partitions)))
            },
        }
    }
    pub fn order(&self, position: Vec2) -> Vec<LineSegment> {
        let side = bsp_point_side_of_line(&self.partition, &position);
        let mut ordered_segments = Vec::new();
        match side {
            Order::Left | Order::On => {
                if let Some(back_node) = &self.back {
                    ordered_segments.extend(back_node.order(position));
                }
                ordered_segments.push(self.partition);

                if let Some(front_node) = &self.front {
                    ordered_segments.extend(front_node.order(position));
                }
            }
            Order::Right => {
                if let Some(front_node) = &self.front {
                    ordered_segments.extend(front_node.order(position));
                }
                ordered_segments.push(self.partition);

                if let Some(back_node) = &self.back {
                    ordered_segments.extend(back_node.order(position));
                }
            }
        }
        ordered_segments
    }
}

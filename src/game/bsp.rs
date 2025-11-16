use binary_space_partition::{Plane, BspNode, PlaneCut};
use super::LineSegment;
use super::lines::{point_side_of_line, Order, split_line};

impl Plane for LineSegment {
    fn cut(&self, other: Self) -> PlaneCut<Self> {
        use Order::*;

        if self.is_aligned(&other) {
            return PlaneCut::Sibling(other);
        }
        // designate a bin for the left side
        let mut left: Vec<LineSegment> = vec![];
        // same for right
        let mut right: Vec<LineSegment> = vec![];

        // cut the other line segment by this segment
        match (point_side_of_line(self, &other.start), point_side_of_line(self, &other.end)) {
            // if both endpoints are on the same side, return the appropriate child
            (Left, Left) => left.push(other),
            (Right, Right) => right.push(other),
            // looks like the line *isn't* on one side. It chose this path, now it must walk it.
            _ => {
                let (left_side, right_side) = split_line(&self, &other);
                left.push(left_side);
                right.push(right_side);
            }
        }

        PlaneCut::Cut { front: left, back: right }
    }
    fn is_aligned(&self, other: &Self) -> bool {
       point_side_of_line(self, &other.start) == Order::On &&
       point_side_of_line(self, &other.end) == Order::On
    }
}

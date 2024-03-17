use crate::math::*;
use crate::query::ClosestPoints;
use crate::shape::Segment;

/// Distance between two segments.
#[inline]
pub fn distance_segment_segment(pos12: &Isometry, segment1: &Segment, segment2: &Segment) -> Real {
    match crate::query::details::closest_points_segment_segment(
        pos12,
        segment1,
        segment2,
        Real::MAX,
    ) {
        ClosestPoints::WithinMargin(p1, p2) => distance(p1, pos12.transform_point(&p2)),
        _ => 0.0,
    }
}

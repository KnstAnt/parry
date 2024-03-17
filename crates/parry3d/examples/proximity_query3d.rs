use parry3d::math::{Isometry, Vector};
use parry3d::query;
use parry3d::shape::{Ball, Cuboid};

fn main() {
    let cuboid = Cuboid::new(Vector::new(1.0, 1.0, 1.0));
    let ball = Ball::new(1.0);
    let cuboid_pos = Isometry::identity();
    let ball_pos_intersecting = Isometry::translation(1.0, 1.0, 1.0);
    let ball_pos_disjoint = Isometry::translation(3.0, 3.0, 3.0);

    let intersecting =
        query::intersection_test(&ball_pos_intersecting, &ball, &cuboid_pos, &cuboid).unwrap();
    let not_intersecting =
        !query::intersection_test(&ball_pos_disjoint, &ball, &cuboid_pos, &cuboid).unwrap();

    assert!(intersecting);
    assert!(not_intersecting);
}

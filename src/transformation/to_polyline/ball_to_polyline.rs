use crate::math::*;
use crate::shape::Ball;
use crate::transformation::utils;
use na::{self, RealField};

impl Ball {
    /// Discretize the boundary of this ball as a polygonal line.
    pub fn to_polyline(&self, nsubdivs: u32) -> Vec<Point> {
        let diameter = self.radius * 2.0;
        let two_pi = Real::two_pi();
        let dtheta = two_pi / (nsubdivs as Real);

        let mut pts = Vec::with_capacity(nsubdivs as usize);
        utils::push_xy_arc(diameter / 2.0, nsubdivs, dtheta, &mut pts);

        pts
    }
}

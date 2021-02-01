//! Source: [Jankauskas, K. (2010). Time-efficient nurbs curve evaluation algorithms. *evaluation*, *1*(5), 8.](https://www.researchgate.net/publication/228411721_Time-Efficient_NURBS_Curve_Evaluation_Algorithms)

use euclid::Vector3D;

/// Represents a nurbs curve.
///
/// Note that the knots vector is normalized (range 0 to 1).
#[derive(Debug)]
pub struct Spline<T> {
    degree: usize,
    c_points: Vec<(f32, Vector3D<f32, T>)>,
    knots: Vec<f32>,
}

impl<T> Spline<T> {
    pub fn new(
        degree: usize,
        c_points: Vec<(f32, Vector3D<f32, T>)>,
        knots: Vec<f32>,
    ) -> Spline<T> {
        Spline {
            degree,
            c_points,
            knots,
        }
    }

    fn get_point_0(&self, bases: &mut [f32], k: usize) -> Vector3D<f32, T> {
        let mut bases_sum = 0.;
        let mut point = Vector3D::new(0., 0., 0.);
        for (i, base) in bases.iter_mut().enumerate().take(self.degree + 1) {
            let c_point = self.c_points[k - self.degree + i];
            *base *= c_point.0;
            bases_sum += *base;
            point += c_point.1 * *base;
        }
        point / bases_sum
    }

    fn _basis_its_0(&self, k: usize, u: f32) -> Vec<f32> {
        let mut bases = vec![0.; self.degree + 1];
        let mut ls = vec![0.; self.degree + 1];
        let mut rs = vec![0.; self.degree + 1];
        self.basis_its_0_no_alloc(k, u, &mut bases[..], &mut ls[..], &mut rs[..]);
        bases
    }

    fn basis_its_0_no_alloc(
        &self,
        k: usize,
        u: f32,
        bases: &mut [f32],
        ls: &mut [f32],
        rs: &mut [f32],
    ) {
        bases[0] = 1.;
        for j in 1..=self.degree {
            let mut saved = 0.;
            ls[j] = u - self.knots[k + 1 - j];
            rs[j] = self.knots[k + j] - u;
            let mut r = 0;
            while r < j {
                let tmp = bases[r] / (rs[r + 1] + ls[j - r]);
                bases[r] = saved + rs[r + 1] * tmp;
                saved = ls[j - r] * tmp;
                r += 1;
            }
            bases[j] = saved;
        }
    }

    pub fn get_points(&self, step_count: usize) -> Vec<Vector3D<f32, T>> {
        // Preallocate buffers. Create a bigger buffer and then split it into
        // three parts to avoid multiple allocations.
        let mut float_buff = vec![0.; (self.degree + 1) * 3];
        let (bases, rest) = float_buff.split_at_mut(self.degree + 1);
        let (ls, rs) = rest.split_at_mut(self.degree + 1);
        let mut points = Vec::with_capacity(step_count);

        let step = 1. / (step_count - 1) as f32;

        points.push(self.c_points[0].1);

        let mut u = self.knots[self.degree] + step;
        let mut k = self.degree;
        while k < self.c_points.len() {
            while (self.knots[k] - self.knots[k + 1]).abs() < f32::EPSILON && self.knots[k] < 1. {
                k += 1;
            }
            while u < self.knots[k + 1] {
                self.basis_its_0_no_alloc(k, u, &mut bases[..], &mut ls[..], &mut rs[..]);
                points.push(self.get_point_0(&mut bases[..], k));
                u += step;
            }
            k += 1;
        }

        points.push(self.c_points[self.c_points.len() - 1].1);

        debug_assert_eq!(points.len(), step_count);

        points
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use euclid::vec3;

    #[test]
    fn it_creates_a_circle() {
        let pi = std::f32::consts::PI;
        let a = Spline::<()> {
            degree: 2,
            c_points: vec![
                (1., vec3(1., 0., 0.)),
                (2_f32.sqrt() / 2., vec3(1., 1., 0.)),
                (1., vec3(0., 1., 0.)),
                (2_f32.sqrt() / 2., vec3(-1., 1., 0.)),
                (1., vec3(-1., 0., 0.)),
                (2_f32.sqrt() / 2., vec3(-1., -1., 0.)),
                (1., vec3(0., -1., 0.)),
                (2_f32.sqrt() / 2., vec3(1., -1., 0.)),
                (1., vec3(1., 0., 0.)),
            ],
            knots: vec![
                0. / 2. / pi,
                0. / 2. / pi,
                0. / 2. / pi,
                pi / 2. / 2. / pi,
                pi / 2. / 2. / pi,
                pi / 2. / pi,
                pi / 2. / pi,
                3. * pi / 2. / 2. / pi,
                3. * pi / 2. / 2. / pi,
                2. * pi / 2. / pi,
                2. * pi / 2. / pi,
                2. * pi / 2. / pi,
            ],
        };

        assert_eq!(
            a.get_points(5),
            vec![
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                vec3(-1.0, 0.0, 0.0),
                vec3(0.0, -1.0, 0.0),
                vec3(1.0, 0.0, 0.0)
            ]
        );
    }
}

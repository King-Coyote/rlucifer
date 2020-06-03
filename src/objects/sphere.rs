use crate::basic_types::*;
use crate::objects::Renderable;


pub struct Sphere {
    id: usize,
    position: Vector3,
    radius: f32,
}

impl Renderable for Sphere {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_normal_at(&self, point: Vector3) -> Vector3 {
        (&point - &self.position).normalise()
    }

    fn get_intersection(&self, ray: &Ray) -> Option<Hit> {
        // setup the B and C of the quadratic equation (At^2 + Bt + C = R^2, but A is 1 since the direction vector is normalised by construction)
        let diff = &ray.origin - &self.position;
        let B = 2.0 * diff.dot(&ray.dir);
        let C = diff.dot(&diff) - (self.radius * self.radius);
        let dis = B*B - 4.0*C;
        if dis < 0.0 {
            None
        } else {
            // 2 cases for the two signs of the equation.
            let soln1 = -B - dis.sqrt();
            let soln2 = -B + dis.sqrt();
            if soln1 > 1e-3 {
                let hit_point = &ray.origin + &(&ray.dir * (soln1/2.0));
                Some(Hit::new(soln1 / 2.0, hit_point.clone(), self.get_normal_at(hit_point)))
            } else if soln2 > 1e-3 {
                let hit_point = &ray.origin + &(&ray.dir * (soln2/2.0));
                Some(Hit::new(soln2 / 2.0, hit_point.clone(), self.get_normal_at(hit_point)))
            } else {
                None
            }
        }
    }

}


use dim3::{Polygon, Vector, Vertex};
use {Unit, EPSILON};

bitflags! {
    struct Location: u32 {
        const NONE = 0;
        const COPLANAR = 0;
        const FRONT = 1;
        const BACK = 2;
        const FRONT_AND_BACK = 3;
    }
}

type Collector = Vec<Polygon>;

/// Represents a plane in 3D space.
#[derive(Clone, Debug)]
pub struct Plane(pub Vector, pub Unit);

impl Plane {
    pub fn from_points(v0: Vector, v1: Vector, v2: Vector) -> Plane {
        let n = (v1 - v0).cross(v2 - v0).normalize();
        Plane(n, n.dot(v0))
    }

    pub fn flip(&self) -> Plane {
        Plane(self.0.negate(), -self.1)
    }

    #[inline]
    pub fn normal(&self) -> Vector {
        self.0
    }

    #[inline]
    pub fn w(&self) -> Unit {
        self.1
    }

    /// Split `polygon` by this plane if needed, then put the polygon or polygon fragments in the
    /// appropriate lists. Coplanar polygons go into either `coplanarFront` or `coplanarBack`
    /// depending on their orientation with respect to this plane. Polygons in front or in back of
    /// this plane go into either `front` or `back`
    pub fn split_polygon(
        &self,
        poly: &Polygon,
        coplanar_front: &mut Collector,
        coplanar_back: &mut Collector,
        front: &mut Collector,
        back: &mut Collector,
    ) {
        let mut polygon_type = Location::NONE;
        let mut vertex_locs: Vec<Location> = Vec::with_capacity(poly.vertices.len());

        for v in poly.vertices.iter() {
            let t = self.normal().dot(v.position) - self.w();

            let loc = {
                if t < -EPSILON {
                    Location::BACK
                } else if t > EPSILON {
                    Location::FRONT
                } else {
                    Location::COPLANAR
                }
            };

            polygon_type |= loc;
            vertex_locs.push(loc);
        }

        match polygon_type {
            Location::COPLANAR => {
                if self.normal().dot(poly.plane.normal()) > 0. {
                    coplanar_front.push(poly.clone());
                } else {
                    coplanar_back.push(poly.clone());
                }
            }
            Location::FRONT => {
                front.push(poly.clone());
            }
            Location::BACK => {
                back.push(poly.clone());
            }
            Location::FRONT_AND_BACK => {
                let mut inner_front: Vec<Vertex> = Vec::new();
                let mut inner_back: Vec<Vertex> = Vec::new();

                for i in 0..poly.vertices.len() {
                    let j = (i + 1) % poly.vertices.len();
                    let ti = vertex_locs[i];
                    let tj = vertex_locs[j];
                    let vi = poly.vertices[i];
                    let vj = poly.vertices[j];

                    if ti != Location::BACK {
                        inner_front.push(vi);
                    }

                    if ti != Location::FRONT {
                        inner_back.push(vi);
                    }

                    if (ti | tj) == Location::FRONT_AND_BACK {
                        let t = (self.w() - self.normal().dot(vi.position))
                            / self.normal().dot(vj.position - vi.position);

                        let v = vi.interpolate(vj, t);
                        inner_front.push(v);
                        inner_back.push(v);
                    }
                }

                if inner_front.len() >= 3 {
                    front.push(Polygon::new(inner_front));
                }

                if inner_back.len() >= 3 {
                    back.push(Polygon::new(inner_back));
                }
            }
            _ => (),
        }
    }
}

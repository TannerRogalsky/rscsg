use dim3::{Csg, Polygon, Vector, Vertex};
use Unit;

impl Csg {
    pub fn cylinder(start: Vector, end: Vector, radius: Unit, slices: usize) -> Csg {
        let s = start;
        let e = end;
        let ray = e - s;
        let axis_z = ray.normalize();
        let is_y = axis_z.y().abs() > 0.5;
        let axis_x = Vector(is_y as u8 as Unit, !is_y as u8 as Unit, 0.)
            .cross(axis_z)
            .normalize();
        let axis_y = axis_x.cross(axis_z).normalize();
        let start = Vertex::new(s, axis_z.negate());
        let end = Vertex::new(e, axis_z.normalize());

        let point = |stack: Unit, slice: Unit, normal_blend: Unit| -> Vertex {
            let angle = slice * crate::PI * 2.;
            let out = (axis_x * angle.cos()) + (axis_y * angle.sin());
            let pos = s + (ray * stack) + (out * radius);
            let normal = out * (1. - normal_blend.abs()) + (axis_z * normal_blend);
            Vertex::new(pos, normal)
        };

        let mut polygons = Vec::with_capacity(slices * 3);
        for i in 0..slices {
            let t0 = i as Unit / slices as Unit;
            let t1 = (i + 1) as Unit / slices as Unit;
            polygons.push(Polygon::new(vec![
                start,
                point(0., t0, -1.),
                point(0., t1, -1.),
            ]));
            polygons.push(Polygon::new(vec![
                point(0., t1, 0.),
                point(0., t0, 0.),
                point(1., t0, 0.),
                point(1., t1, 0.),
            ]));
            polygons.push(Polygon::new(vec![
                end,
                point(1., t1, 1.),
                point(1., t0, 1.),
            ]));
        }
        Self::from_polygons(polygons)
    }
}

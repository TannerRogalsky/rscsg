use dim3::Csg;
use dim3::{Polygon, Vector, Vertex};
use {Unit, UNIT_PI};

impl Csg {
    pub fn sphere(center: Vector, radius: Unit, slices: usize, stacks: usize) -> Csg {

        let mut vertex = |theta: Unit, phi: Unit| -> Vertex {
            let theta = theta * UNIT_PI * 2.;
            let phi = phi * UNIT_PI;
            let dir = Vector(
                theta.cos() * phi.sin(),
                phi.cos(),
                theta.sin() * phi.sin()
            );
            Vertex::new(center + dir * radius, dir)
        };

        let mut polys = vec![];
        for slice in 0..slices {
            for stack in 0..stacks {
                let i = slice as Unit;
                let j = stack as Unit;
                let fslices = slices as Unit;
                let fstacks = stacks as Unit;

                let mut vertices = vec![];
                vertices.push(vertex(i / fslices, j / fstacks));
                if stack > 0 {
                    vertices.push(vertex((i + 1.) / fslices, j / fstacks));
                }
                if stack < stacks - 1 {
                    vertices.push(vertex((i + 1.) / fslices, (j + 1.) / fstacks))
                }
                vertices.push(vertex(i / fslices, (j + 1.) / fstacks));
                polys.push(Polygon::new(vertices));
            }
        }

        Csg::from_polygons(polys)
    }
}

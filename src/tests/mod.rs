mod bounding_box;
mod dim2;
mod plane;

use self::bounding_box::BoundBox;
use dim3::{BspNode, Csg, Plane, Polygon, Vector, Vertex};

#[test]
fn types() {
    Csg::new();
    BspNode::new(Some(vec![]));
    Plane::from_points(Vector(0., 0., 0.), Vector(1., 0., 0.), Vector(0., 1., 0.));
    Polygon::new(vec![
        Vertex::new(Vector(0., 0., 0.), Vector(0., 0., 1.)),
        Vertex::new(Vector(1., 0., 0.), Vector(0., 0., 1.)),
        Vertex::new(Vector(0., 1., 0.), Vector(0., 0., 1.)),
    ]);
    Vector(0., 0., 0.);
    Csg::new();
}

/// Create a cube, make sure it's inside a bounding box.
#[test]
fn csg_cube() {
    let cube = Csg::cube(Vector(2., 2., 2.), true);
    let bb = BoundBox::from_csg(&cube);

    // Get ivectors of bounding box, coords snapped to closest 0.1
    let (d_min, d_max) = bb.get_min_max_discreet(10.);

    assert_eq!(-10, d_min.0);
    assert_eq!(-10, d_min.1);
    assert_eq!(-10, d_min.2);
    assert_eq!(10, d_max.0);
    assert_eq!(10, d_max.1);
    assert_eq!(10, d_max.2);
}

/// Big cube will subtract itself onto a smaller cube, removing everything.
#[test]
fn csg_total_subtraction() {
    let polys = Csg::subtract(
        &Csg::cube(Vector(1., 1., 1.), true), // Small cube
        &Csg::cube(Vector(2., 2., 2.), true), // Big cube
    )
    .to_polygons();

    assert_eq!(0, polys.len());
}

#[test]
fn csg_sphere() {
    let sphere = Csg::sphere(Vector(0., 0., 0.), 1.0, 10, 5);
    let bb = BoundBox::from_csg(&sphere);

    let (d_min, d_max) = bb.get_min_max_discreet(10.);

    assert_eq!(-10, d_min.0);
    assert_eq!(-10, d_min.1);
    assert_eq!(-9, d_min.2);
    assert_eq!(10, d_max.0);
    assert_eq!(10, d_max.1);
    assert_eq!(9, d_max.2);
}

#[test]
fn stack_overflow_regression() {
    fn scene_cube(_step: i32) -> Csg {
        Csg::cube(Vector(1., 1., 1.), true)
    }

    fn scene_cylinder(step: i32) -> Csg {
        let arm_length = 2.;
        let radius = 0.5;
        let rotate = 30. + (step * 4) as f32;
        let slices = 8;
        Csg::union(
            &Csg::cylinder(
                Vector(-arm_length, 0., 0.),
                Vector(arm_length, 0., 0.),
                radius,
                slices,
            ),
            &Csg::cylinder(
                Vector(0., -arm_length, 0.),
                Vector(0., arm_length, 0.),
                radius,
                slices,
            ),
        )
        .rotate(Vector(1., 0., 0.), rotate)
    }

    fn scene_cylinder_sub_cube(step: i32) -> Csg {
        Csg::subtract(&scene_cube(0), &scene_cylinder(step))
    }

    println!("test");
    let output = scene_cylinder_sub_cube(49);
    assert!(!output.polygons.is_empty());
}

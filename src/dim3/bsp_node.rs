use dim3::{Plane, Polygon};

/// Holds a node in a BSP tree. A BSP tree is built from a collection of polygons by picking a
/// polygon to split along. That polygon (and all other coplanar polygons) are added directly to
/// that node and the other polygons are added to the front and/or back subtrees. This is not a
/// leafy BSP tree since there is no distinction between internal and leaf nodes.

#[derive(Clone)]
pub struct BspNode {
    pub plane: Option<Plane>,
    pub front: Option<Box<BspNode>>,
    pub back: Option<Box<BspNode>>,
    pub polygons: Vec<Polygon>,
}

impl BspNode {
    pub fn new(polygons: Option<Vec<Polygon>>) -> BspNode {
        let mut bsp = BspNode {
            plane: None,
            front: None,
            back: None,
            polygons: Vec::new(),
        };

        if let Some(polygons) = polygons {
            bsp.build(polygons);
        }
        bsp
    }

    /// Convert solid space to empty space and empty space to solid space.
    pub fn invert(&mut self) {
        for p in self.polygons.iter_mut() {
            p.flip();
        }

        if let Some(plane) = &mut self.plane {
            *plane = plane.flip();
        }

        if let Some(front) = &mut self.front {
            front.invert();
        }

        if let Some(back) = &mut self.back {
            back.invert();
        }

        std::mem::swap(&mut self.front, &mut self.back);
    }

    /// Recursively remove all polygons in `polygons` that are inside this BSP tree.
    pub fn clip_polygons(&self, polygons: &[Polygon]) -> Vec<Polygon> {
        let plane = match self.plane.as_ref() {
            Some(plane) => plane,
            None => return self.polygons.clone(),
        };

        let mut front: Vec<Polygon> = Vec::new();
        let mut back: Vec<Polygon> = Vec::new();

        for poly in polygons {
            let mut second_front: Vec<Polygon> = Vec::new();
            let mut second_back: Vec<Polygon> = Vec::new();
            plane.split_polygon(
                &poly,
                &mut front,
                &mut back,
                &mut second_front,
                &mut second_back,
            );
            front.append(&mut second_front);
            back.append(&mut second_back);
        }

        let mut front = if let Some(own_front) = &self.front {
            own_front.clip_polygons(&front)
        } else {
            front
        };

        let mut back = if let Some(own_back) = &self.back {
            own_back.clip_polygons(&back)
        } else {
            Vec::new()
        };

        front.append(&mut back);
        front
    }

    pub fn clip_to(&mut self, bsp: &BspNode) {
        self.polygons = bsp.clip_polygons(&self.polygons);

        if let Some(front) = &mut self.front {
            front.clip_to(bsp);
        }

        if let Some(back) = &mut self.back {
            back.clip_to(bsp);
        }
    }

    pub fn all_polygons(&self) -> Vec<Polygon> {
        let mut polys: Vec<Polygon> = Vec::new();
        self.fill_polygons(&mut polys);
        polys
    }

    fn fill_polygons(&self, polys: &mut Vec<Polygon>) {
        polys.append(&mut self.polygons.clone());

        if let Some(front) = &self.front {
            front.fill_polygons(polys);
        }

        if let Some(back) = &self.back {
            back.fill_polygons(polys);
        }
    }

    /// Build a BSP tree out of `Vec<Polygon>`. When called on an existing tree, the new polygons
    /// are filtered down to the bottom of the tree and become new nodes there. Each set of
    /// polygons is partitioned using the first polygon (no heuristic is used to pick a good
    /// split).
    pub fn build(&mut self, polygons: Vec<Polygon>) {
        if polygons.is_empty() {
            return;
        }

        let plane = self.plane.get_or_insert_with(|| polygons[0].plane.clone());

        let mut front: Vec<Polygon> = Vec::new();
        let mut back: Vec<Polygon> = Vec::new();

        for poly in polygons.iter() {
            let mut second: Vec<Polygon> = Vec::new();

            plane.split_polygon(
                &poly,
                &mut self.polygons,
                &mut second,
                &mut front,
                &mut back,
            );
            self.polygons.append(&mut second);
        }

        // Recursively build the BSP tree

        if !front.is_empty() {
            let this_front = self.front.get_or_insert(Box::new(BspNode::new(None)));
            this_front.build(front);
        }

        if !back.is_empty() {
            let this_back = self.back.get_or_insert(Box::new(BspNode::new(None)));
            this_back.build(back);
        }
    }
}

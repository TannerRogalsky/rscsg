use dim2::{Line, Plane};

#[derive(Clone)]
pub struct BspNode {
    pub plane: Option<Plane>,
    pub front: Option<Box<BspNode>>,
    pub back: Option<Box<BspNode>>,
    pub lines: Vec<Line>,
}

impl BspNode {
    pub fn new(lines: Option<Vec<Line>>) -> BspNode {
        let mut bsp = BspNode {
            plane: None,
            front: None,
            back: None,
            lines: Vec::new(),
        };

        if let Some(lines) = lines {
            bsp.build(lines);
        }
        bsp
    }

    pub fn invert(&mut self) {
        for l in self.lines.iter_mut() {
            l.flip();
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

        {
            let temp = self.front.clone();
            self.front = self.back.clone();
            self.back = temp;
        }
    }

    pub fn clip_lines(&mut self, lines: &[Line]) -> Vec<Line> {
        if let Some(plane) = &mut self.plane {
            let mut front: Vec<Line> = Vec::new();
            let mut back: Vec<Line> = Vec::new();

            for line in lines {
                let mut second_front: Vec<Line> = Vec::new();
                let mut second_back: Vec<Line> = Vec::new();
                plane.split_lines(
                    *line,
                    &mut front,
                    &mut back,
                    &mut second_front,
                    &mut second_back,
                );
                front.append(&mut second_front);
                back.append(&mut second_back);
            }

            let mut front = if let Some(this_front) = &mut self.front {
                this_front.clip_lines(&front)
            } else {
                front
            };

            let mut back = if let Some(this_back) = &mut self.back {
                this_back.clip_lines(&back)
            } else {
                Vec::new()
            };

            front.append(&mut back);
            front
        } else {
            self.lines.clone()
        }
    }

    pub fn clip_to(&mut self, bsp: &mut BspNode) {
        self.lines = bsp.clip_lines(&self.lines);

        if let Some(front) = &mut self.front {
            front.clip_to(bsp);
        }

        if let Some(back) = &mut self.back {
            back.clip_to(bsp);
        }
    }

    pub fn all_lines(&self) -> Vec<Line> {
        let mut lines: Vec<Line> = self.lines.clone();
        self.fill_lines(&mut lines);
        lines
    }

    fn fill_lines(&self, lines: &mut Vec<Line>) {
        lines.append(&mut self.lines.clone());

        if let Some(front) = &self.front {
            front.fill_lines(lines);
        }

        if let Some(back) = &self.back {
            back.fill_lines(lines);
        }
    }

    pub fn build(&mut self, lines: Vec<Line>) {
        if lines.is_empty() {
            return;
        }

        let plane = self.plane.get_or_insert(lines[0].plane);

        self.lines.push(lines[0]);
        let mut front: Vec<Line> = Vec::new();
        let mut back: Vec<Line> = Vec::new();

        for line in lines.iter().skip(1) {
            let mut second: Vec<Line> = Vec::new();

            plane.split_lines(*line, &mut self.lines, &mut second, &mut front, &mut back);
            self.lines.append(&mut second);
        }

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

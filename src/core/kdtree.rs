use geometry::{ BBox, Point, distance_squared, Union };

pub struct KdNode {
    split_pos:      f32,
    split_axis:     uint,
    has_left_child: bool,
    right_child:    uint
}

impl KdNode {
    pub fn init(&mut self, p: f32, a: uint) {
        self.split_pos      = p;
        self.split_axis     = a;
        self.right_child    = (1 << 29) - 1;
        self.has_left_child = false;
    }

    pub fn init_leaf(&mut self) {
        self.split_axis     = 3;
        self.right_child    = (1 << 29) - 1;
        self.has_left_child = false;
    }
}

pub struct KdTree<T> {
    nodes:          Vec<KdNode>,
    node_data:      Vec<T>,
    next_free_node: uint,
    number_nodes:   uint
}

pub trait KdNodeData {
    fn get_point(&self) -> Point;
}

impl<T: KdNodeData + Clone> KdTree<T> {
    pub fn new(d: &Vec<T>) -> KdTree<T> {
        let mut tree = KdTree {
            nodes: Vec::new(),
            node_data: Vec::new(),
            next_free_node: 1,
            number_nodes: d.len()
        };

        tree.recursive_build(0, 0, tree.number_nodes, d);

        return tree;
    }

    fn recursive_build(&mut self, node_num: uint, start: uint, end: uint, build_nodes: &Vec<T>) {
        if start + 1 == end {
            self.nodes.get_mut(node_num).init_leaf();
            self.node_data.grow_set(node_num, build_nodes.get(start), 
                build_nodes.get(start).clone());
            return;
        }

        let mut bound = BBox::from_point(&Point::zero());
        for i in range(start, end) {
            bound = bound.union(&build_nodes.get(i).get_point());
        }

        let split_axis = bound.maximum_extent();
        let split_pos  = (start + end) / 2;

        // TODO: n-th element

        self.nodes.get_mut(node_num).init(
            build_nodes.get(split_pos).get_point()[split_axis], split_axis);
        self.node_data.grow_set(node_num, build_nodes.get(split_pos), 
            build_nodes.get(split_pos).clone());

        if start < split_pos {
            self.nodes.get_mut(node_num).has_left_child = true;
            let child_num = self.next_free_node;
            self.next_free_node += 1;
            self.recursive_build(child_num, start, split_pos, build_nodes);
        }

        if split_pos + 1 < end {
            self.nodes.get_mut(node_num).right_child = self.next_free_node;
            self.next_free_node += 1;
            let right = self.nodes.get(node_num).right_child;
            self.recursive_build(right, split_pos + 1, end, build_nodes);   
        }
    }

    pub fn lookup(&self, node_num: uint, p: &Point, max_dist_squared: &mut f32, 
            process: |&Point, &T, f32, f32| -> bool) {
        self.lookup_private(0, p, max_dist_squared, process);
    }

    fn lookup_private(&self, node_num: uint, p: &Point, max_dist_squared: &mut f32, 
            process: |&Point, &T, f32, f32| -> bool) {
        let node = self.nodes.get(node_num);
        let axis = node.split_axis;

        if axis != 3 {
            let dist2 = (p[axis] - node.split_pos) * (p[axis] - node.split_pos);
            if p[axis] <= node.split_pos {
                if node.has_left_child {
                    self.lookup_private(node_num + 1, p, max_dist_squared, 
                        |a, b, c, d| process(a, b, c, d));
                }
                if dist2 < *max_dist_squared && node.right_child < self.number_nodes {
                    self.lookup_private(node.right_child, p, max_dist_squared, 
                        |a, b, c, d| process(a, b, c, d));
                }
            } else {
                if node.right_child < self.number_nodes {
                    self.lookup_private(node.right_child, p, max_dist_squared, 
                        |a, b, c, d| process(a, b, c, d));
                }
                if dist2 < *max_dist_squared && node.has_left_child {
                    self.lookup_private(node_num + 1, p, max_dist_squared, 
                        |a, b, c, d| process(a, b, c, d));
                }
            }
        }

        let dist2 = distance_squared(&self.node_data.get(node_num).get_point(), p);
        if dist2 < *max_dist_squared {
            process(p, self.node_data.get(node_num), dist2, *max_dist_squared);
        }
    }
}
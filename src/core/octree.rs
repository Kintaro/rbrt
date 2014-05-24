use geometry::{ BBox, Point, distance_squared };

#[deriving(Clone)]
pub struct OctNode<T> {
  pub children: Vec<Option<OctNode<T>>>,
  pub data:     Vec<T>
}

impl<T: Clone> OctNode<T> {
  pub fn new() -> OctNode<T> {
    OctNode { children: Vec::from_elem(8, None), data: Vec::new() }
  }
}

pub struct Octree<T> {
  pub max_depth: uint,
  pub bound:   BBox,
  pub root:    OctNode<T>
}

impl<T: Clone> Octree<T> {
  pub fn new(b: BBox) -> Octree<T> {
    Octree::with_depth(b, 16)
  }

  pub fn with_depth(b: BBox, d: uint) -> Octree<T> {
    Octree { bound: b, max_depth: d, root: OctNode::new() }
  }

  pub fn add(&mut self, data_item: T, data_bound: &BBox) {
    Octree::add_private(self.max_depth, &mut self.root, &self.bound, data_item, data_bound,
      distance_squared(&data_bound.p_min, &data_bound.p_max), 0);
  }

  pub fn lookup(&self, p: &Point, process: |&T| -> bool) {
    if self.bound.inside(p) {
      self.lookup_private(&self.root, &self.bound, p, process);
    }
  }

  fn add_private(max_depth: uint, node: &mut OctNode<T>, node_bound: &BBox,
    data_item: T, data_bound: &BBox, diag2: f32, depth: uint) {

    if depth == max_depth || distance_squared(&node_bound.p_min, &node_bound.p_max) < diag2 {
      node.data.push(data_item);
      return;
    }

    let pmid = node_bound.p_min * 0.5 + node_bound.p_max * 0.5;

    let x = [ data_bound.p_min.x <= pmid.x, data_bound.p_max.x > pmid.x ];
    let y = [ data_bound.p_min.y <= pmid.y, data_bound.p_max.y > pmid.y ];
    let z = [ data_bound.p_min.z <= pmid.z, data_bound.p_max.z > pmid.z ];

    let over = [
      x[0] && y[0] && z[0],
      x[0] && y[0] && z[1],
      x[0] && y[1] && z[0],
      x[0] && y[1] && z[1],
      x[1] && y[0] && z[0],
      x[1] && y[0] && z[1],
      x[1] && y[1] && z[0],
      x[1] && y[1] && z[1]
    ];

    for i in range(0u, 8) {
      if !over[i] {
        continue;
      }

      match *node.children.get(i) {
        None => {
          let mut n = OctNode::new();

          let child_bound = octree_child_bound(i, node_bound, &pmid);
          Octree::add_private(max_depth, &mut n,
            &child_bound, data_item.clone(), data_bound, diag2, depth + 1);

          node.children.grow_set(i, &Some(OctNode::new()), Some(n));
        },
        _  => ()
      };
    }
  }

  fn lookup_private(&self, node: &OctNode<T>, node_bound: &BBox, p: &Point, process: |&T| -> bool) -> bool {
    for d in node.data.iter() {
      if process(d) {
        return false;
      }
    }

    let pmid = node_bound.p_min * 0.5 + node_bound.p_max * 0.5;
    let child =
      if p.x > pmid.x { 4 } else { 0 } +
      if p.y > pmid.y { 2 } else { 0 } +
      if p.z > pmid.z { 1 } else { 0 };

    if node.children.get(child).is_none() {
      return true;
    }

    match *node.children.get(child) {
      None    => true,
      Some(ref x) => {
        let child_bound = octree_child_bound(child, node_bound, &pmid);
        self.lookup_private(x, &child_bound, p, process)
      }
    }
  }
}

fn octree_child_bound(child: uint, node_bound: &BBox, pmid: &Point) -> BBox {
  let mut child_bound = node_bound.clone();

  child_bound.p_min.x = if (child & 4) != 0 { pmid.x } else { node_bound.p_min.x };
  child_bound.p_min.y = if (child & 2) != 0 { pmid.y } else { node_bound.p_min.y };
  child_bound.p_min.z = if (child & 1) != 0 { pmid.z } else { node_bound.p_min.z };

  child_bound.p_max.x = if (child & 4) != 0 { node_bound.p_max.x } else { pmid.x };
  child_bound.p_max.y = if (child & 2) != 0 { node_bound.p_max.y } else { pmid.y };
  child_bound.p_max.z = if (child & 1) != 0 { node_bound.p_max.z } else { pmid.z };

  return child_bound;
}

use geometry::{ Point, Vector, Normal };

#[deriving(Clone)]
pub struct ParamSetItem<T> {
  name: StrBuf,
  data: Vec<T>,
  looked_up: bool
}

pub struct ParamSet {
  bools: Vec<ParamSetItem<bool>>,
  ints: Vec<ParamSetItem<int>>,
  floats: Vec<ParamSetItem<f32>>,
  points: Vec<ParamSetItem<Point>>,
  vectors: Vec<ParamSetItem<Vector>>,
  normals: Vec<ParamSetItem<Normal>>
}

impl ParamSet {
  pub fn add_float(&mut self, name: &StrBuf, data: Vec<f32>) {
    self.erase_float(name);
    self.floats.push(ParamSetItem { name: name.clone(), data: data, looked_up: false });
  }

  pub fn erase_float(&mut self, name: &StrBuf) {
    self.floats.retain(|x| x.name != *name);
  }
}

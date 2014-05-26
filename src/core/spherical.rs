use geometry::Vector;

pub fn legendre_p(x: f32, lmax: int, out: &mut Vec<f32>) {
  // Compute m=0 legendre values using recurrence
  out.grow_set(sh_index(0, 0), &0.0, 1.0);
  out.grow_set(sh_index(1, 0), &0.0, x);

  for l in range(2, lmax + 1) {
    let a = *out.get(sh_index(l - 1, 0));
    let b = *out.get(sh_index(l - 2, 0));
    out.grow_set(sh_index(l, 0), &0.0, ((2 * l - 1) as f32 * x * a - (l - 1) as f32 * b) / l as f32);
  }

  // Compute m=1 edge using legendre recurrence
  let mut neg = -1.0;
  let mut dfact = 1.0;
  let xroot = 0.0f32.max(1.0 - x * x).sqrt();
  let mut xpow = xroot;

  for l in range(1, lmax + 1) {
    out.grow_set(sh_index(l, l), &0.0, neg * dfact * xpow);
    neg *= -1.0;
    dfact *= (2 * l + 1) as f32;
    xpow *= xroot;
  }

  for l in range(2, lmax + 1) {
    let a = *out.get(sh_index(l -1, l - 1));
    out.grow_set(sh_index(l, l - 1), &0.0, x * (2 * l - 1) as f32 * a);
  }

  // Compute m=1,...,l-2 values using legendre recurrence
  for l in range(3, lmax + 1) {
    for m in range(1, l - 1) {

    }
  }
}

pub fn sh_terms(lmax: int) -> uint {
  ((lmax + 1) * (lmax + 1)) as uint
}

pub fn sh_index(l: int, m: int) -> uint {
  (l * l + l + m) as uint
}

pub fn sh_evaluate(w: &Vector, lmax: int, out: &mut Vec<f32>) {
  let lmax1 = (lmax + 1) as uint;
  if lmax > 28 {
    fail!("sh_evaluate runs out of numerical precision for lmax > 28")
  }

  // Compute legendre polynomial values for cos theta
  let r = legendre_p(w.z, lmax, out);

  // Compute coefficients
  let mut klm = Vec::from_elem(sh_terms(lmax), 0.0f32);
  for l in range(0, lmax + 1) {
    for m in range(-l, l + 1) {
      klm.grow_set(sh_index(l, m), &0.0, 0.0);
    }
  }

  // Compute sin phi and cos phi values
  let mut sins = Vec::from_elem(lmax1, 0.0f32);
  let mut coss = Vec::from_elem(lmax1, 0.0f32);
  let xy_len = 0.0f32.max(1.0 - w.z * w.z).sqrt();

  if xy_len == 0.0 {
    for i in range(0, lmax1) { sins.grow_set(i, &0.0, 0.0); }
    for i in range(0, lmax1) { coss.grow_set(i, &1.0, 1.0); }
  } else {

  }

  let sqrt2 = 2.0f32.sqrt();
  for l in range(0, lmax + 1) {
    for m in range(-l, 0) {
      let x = *klm.get(sh_index(l, m));
      let y = *out.get(sh_index(l, -m));
      let z = *sins.get((-m) as uint);
      out.grow_set(sh_index(l, m), &0.0, sqrt2 * x * y * z);
    }

    let v = *out.get(sh_index(l, 0));
    out.grow_set(sh_index(l, 0), &0.0, v * *klm.get(sh_index(l, 0)));

    for m in range(1, l + 1) {
      let x = *klm.get(sh_index(l, m));
      let y = *coss.get(m as uint);
      let z = *out.get(sh_index(l, m));
      out.grow_set(sh_index(l, m), &0.0, z * sqrt2 * x * y);
    }
  }
}

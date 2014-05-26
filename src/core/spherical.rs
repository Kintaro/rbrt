use geometry::Vector;

pub fn legendre_p(x: f32, lmax: int, out: &mut [f32]) {
  // Compute m=0 legendre values using recurrence
  out[sh_index(0, 0)] = 1.0;
  out[sh_index(1, 0)] = x;

  for l in range(2, lmax + 1) {
    let a = out[sh_index(l - 1, 0)];
    let b = out[sh_index(l - 2, 0)];
    out[sh_index(l, 0)] = ((2 * l - 1) as f32 * x * a - (l - 1) as f32 * b) / l as f32;
  }

  // Compute m=1 edge using legendre recurrence
  let mut neg = -1.0;
  let mut dfact = 1.0;
  let xroot = 0.0f32.max(1.0 - x * x).sqrt();
  let mut xpow = xroot;

  for l in range(1, lmax + 1) {
    out[sh_index(l, l)] = neg * dfact * xpow;
    neg   *= -1.0;
    dfact *= (2 * l + 1) as f32;
    xpow  *= xroot;
  }

  for l in range(2, lmax + 1) {
    let a = out[sh_index(l -1, l - 1)];
    out[sh_index(l, l - 1)] = x * (2 * l - 1) as f32 * a;
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

pub fn sh_evaluate(w: &Vector, lmax: int, out_v: &mut Vec<f32>) {
  let lmax1 = (lmax + 1) as uint;
  if lmax > 28 {
    fail!("sh_evaluate runs out of numerical precision for lmax > 28")
  }

  // Compute legendre polynomial values for cos theta
  let len = out_v.len();
  let out = out_v.mut_slice(0, len);
  let r = legendre_p(w.z, lmax, out);

  // Compute coefficients
  let mut klm_v = Vec::from_elem(sh_terms(lmax), 0.0f32);
  let mut klm = klm_v.mut_slice(0, sh_terms(lmax));
  for l in range(0, lmax + 1) {
    for m in range(-l, l + 1) {
      // klm.grow_set(sh_index(l, m), &0.0, 0.0);
      klm[sh_index(l, m)] = 0.0;
    }
  }

  // Compute sin phi and cos phi values
  let mut sins_v = Vec::from_elem(lmax1, 0.0f32);
  let mut coss_v = Vec::from_elem(lmax1, 0.0f32);
  let mut sins = sins_v.mut_slice(0, lmax1);
  let mut coss = coss_v.mut_slice(0, lmax1);
  let xy_len = 0.0f32.max(1.0 - w.z * w.z).sqrt();

  if xy_len == 0.0 {
    for i in range(0, lmax1) { sins[i] = 0.0; }
    for i in range(0, lmax1) { coss[i] = 1.0; }
  } else {

  }

  let sqrt2 = 2.0f32.sqrt();
  for l in range(0, lmax + 1) {
    for m in range(-l, 0) {
      let x = klm[sh_index(l, m)];
      let y = out[sh_index(l, -m)];
      let z = sins[(-m) as uint];
      out[sh_index(l, m)] = sqrt2 * x * y * z;
    }

    out[sh_index(l, 0)] *= klm[sh_index(l, 0)];

    for m in range(1, l + 1) {
      let x = klm[sh_index(l, m)];
      let y = coss[m as uint];
      let z = out[sh_index(l, m)];
      out[sh_index(l, m)] *= sqrt2 * x * y;
    }
  }
}

pub fn k(l: int, m: int) -> f32 {
  let x = (2.0 * l as f32 + 1.0) * divfact(l, m);
  x.sqrt()
}

pub fn divfact(a: int, b: int) -> f32 {
  if b == 0 {
    return 1.0;
  }

  let fa = a;
  let fb = b.abs();
  let v  = 1.0;

  1.0 / v
}

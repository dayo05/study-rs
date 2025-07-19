fn magnitude(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

pub fn normalize(v: [f64; 3]) -> [f64; 3] {
    let mag = magnitude(v);
    [v[0] / mag, v[1] / mag, v[2] / mag]
}

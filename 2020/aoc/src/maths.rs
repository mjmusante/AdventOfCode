
pub fn mod_inv(num: i128, modulus: i128) -> Option<i128> {
    let mdl = modulus.abs();
    let val = if num < 0 { mdl - (-num % mdl) } else { num };

    let mut t = (0, 1);
    let mut r = (mdl, val % mdl);
    while r.1 != 0 {
        let q = r.0 / r.1;
        t = (t.1, t.0 - q * t.1);
        r = (r.1, r.0 - q * r.1);
    }

    if r.0 > 1 {
        return None;
    }
    if t.0 < 0 {
        t.0 += mdl;
    }
    Some(t.0)
}

pub fn pretty_int(n: u64) -> String {
    let mut out    = n.to_string();
    let l = out.len();
    
    let mut i: usize = 3;
    while i < l {
	out.insert(l-i, ',');
	i += 3
    }
    return out
}

pub fn clamp<T: std::cmp::PartialOrd>(x: T, l: T, h: T) -> T {
    match (x < l, x > h) {
        (true, _) => l,
        (_, true) => h,
        _ => x,
    }
}

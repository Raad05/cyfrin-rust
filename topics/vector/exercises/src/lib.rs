pub fn init(x: u32, y: u32, z: u32) -> Vec<u32> {
    // the easy way
    // let v = vec![x, y, z];

    // the hard way
    let mut v: Vec<u32> = Vec::new();
    v.push(x);
    v.push(y);
    v.push(z);

    v
}

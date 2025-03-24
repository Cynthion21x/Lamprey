pub fn in_range(value: u32, lower: u32, upper: u32) -> bool {
    value > lower && value < upper
}

pub fn tuple_add(tuple: (u32, u32), tuple2: (u32, u32)) -> (u32, u32) {
    (tuple.0 + tuple2.0, tuple.1 + tuple2.1)
}

pub fn tuple_times(tuple: (u32, u32), multiplyer: f32) -> (u32, u32) {
    ((tuple.0 as f32* multiplyer) as u32, (tuple.1 as f32 * multiplyer) as u32)
}

pub fn center_x(object: (u32, u32), boundary: (u32, u32), y_pos: u32) -> (u32, u32) {
    ((boundary.0 - object.0) / 2, y_pos)
}

pub fn in_range(value: u32, lower: u32, upper: u32) -> bool {
    value > lower && value < upper
}

pub fn in_range_f32(value: f32, lower: f32, upper: f32) -> bool {
    value > lower && value < upper
}

pub fn tuple_add(tuple: (u32, u32), tuple2: (u32, u32)) -> (u32, u32) {
    (tuple.0 + tuple2.0, tuple.1 + tuple2.1)
}

pub fn tuple_add_f32(tuple: (f32, f32), tuple2: (f32, f32)) -> (f32, f32) {
    (tuple.0 + tuple2.0, tuple.1 + tuple2.1)
}

pub fn tuple_add_i32(tuple: (i32, i32), tuple2: (i32, i32)) -> (i32, i32) {
    (tuple.0 + tuple2.0, tuple.1 + tuple2.1)
}

pub fn tuple_times(tuple: (f32, f32), multiplyer: f32) -> (f32, f32) {
    (tuple.0 * multiplyer, tuple.1 * multiplyer)
}

pub fn center_x(object: (u32, u32), boundary: (u32, u32), y_pos: u32) -> (u32, u32) {
    ((boundary.0 - object.0) / 2, y_pos)
}

pub fn collision(objectpos: (f32, f32), objectsize: (u32, u32), playerpos: (f32, f32), playersize: (u32, u32)) -> bool{
    
    if in_range_f32(playerpos.1, objectpos.1 - playersize.1 as f32, objectpos.1 + objectsize.1 as f32) {
        true
    } else if in_range_f32(playerpos.0, objectpos.0 - playersize.0 as f32 / 2.0, objectpos.0 + objectsize.0 as f32) {
        true
    } else {
        false
    }
    
}

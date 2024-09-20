mod planar_encoding;
use std::collections::LinkedList;
mod linear_encoding;
mod def;
use planar_encoding::{two_dimentional_decoder, two_dimentional_encoder};
fn main() {
   let list_of_points: LinkedList<(u32, u32)> = LinkedList::from([(2,2), (9,1), (4,5), (8,3), (2,1), (4,2), (3,3), (2,4)]);
   let bounding_box = ((2,9),(1,5));
   let point_set_size = list_of_points.len();
   let cell = two_dimentional_encoder::Cell{width : bounding_box.0, height : bounding_box.1, elems : list_of_points};
   let encoding = two_dimentional_encoder::encode_planar_segment(cell);
   println!("Compression Ratio = {}", ((point_set_size * size_of::<(u32, u32)>()) as f32) / ((encoding.queue.len() * size_of::<u8>()) as f32) );
   let decoded = two_dimentional_decoder::decode_planar_segment(encoding, bounding_box);
   decoded.iter().for_each(|(a,b)|{print!("({}, {}) ", a, b)});
}

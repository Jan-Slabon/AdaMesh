use std::collections::LinkedList;
mod def;
mod cubical_encoding;
use cubical_encoding::{three_dimentional_decoder, three_dimentional_encoder};
fn main() {
   let list_of_points: LinkedList<(u32, u32, u32)> = LinkedList::from([(2,2,0), (9,1,5), (4,5,3), (8,3,3), (2,1,1), (4,2,2), (3,3,0), (2,4,5)]);
   let bounding_box = ((2,9),(1,5),(0,5));
   let point_set_size = list_of_points.len();
   let cell = three_dimentional_encoder::Cell{width : bounding_box.0, height : bounding_box.1, depth : bounding_box.2, elems : list_of_points};
   let encoding = three_dimentional_encoder::encode_cubical_segment(cell);
   println!("Compression Ratio = {}", ((point_set_size * size_of::<(u32, u32, u32)>()) as f32) / ((encoding.queue.len() * size_of::<u8>()) as f32) );
   let decoded = three_dimentional_decoder::decode_cubical_segment(encoding, bounding_box);
   decoded.iter().for_each(|(a,b, c)|{print!("({}, {}, {}) ", a, b, c)});
}

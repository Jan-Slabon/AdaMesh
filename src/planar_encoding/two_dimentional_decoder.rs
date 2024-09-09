use core::fmt;
use std::collections::LinkedList;
use std::fmt::{write, Display};

pub struct DCell{
    pub width : (u32,u32),
    pub height : (u32,u32),
    pub num_elems : usize
}
impl Display for DCell{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "width : {} - {}, height : {} - {}, num_of_points : {}", self.width.0, self.width.1, self.height.0, self.height.1, self.num_elems)
    }
}
fn is_any_cell_divisable(list : &LinkedList<DCell>) -> bool
{
    list.iter().any(|DCell{width : (left, right),height : (bottom, top), num_elems : _}|{ right - left > 0 || top - bottom > 0})
}
pub fn decode_planar_segment(mut a : LinkedList<usize>, bounding_box : ((u32, u32), (u32, u32))) -> LinkedList<(u32, u32)>
{
    let mut queue : LinkedList<DCell> = LinkedList::new();
    queue.push_back(DCell{width : bounding_box.0, height : bounding_box.1, num_elems : a.pop_front().unwrap()});
    while is_any_cell_divisable(&queue)
    {
        let DCell{width : (left, right),height : (bottom, top), num_elems : n} = queue.pop_front().unwrap();
        if right - left > 0 || top - bottom > 0
        {
            let points_on_segment_l = a.pop_front().unwrap();
            let points_on_segment_r = n - points_on_segment_l;
            let s0 : DCell;
            let s1 : DCell;
            if right - left >= top - bottom
            {
                s0 = DCell{width : (left, (right - left)/2 + left), height : (bottom, top), num_elems : points_on_segment_l};
                s1 = DCell{width : ((right - left)/2 + 1 + left, right), height : (bottom, top), num_elems : points_on_segment_r};   
            }
            else
            {
                s0 = DCell{width : (left, right), height : (bottom, (top - bottom)/2 + bottom), num_elems : points_on_segment_l};
                s1 = DCell{width : (left, right), height : ((top - bottom)/2 + bottom + 1, top), num_elems : points_on_segment_r};
            }
            if points_on_segment_l > 0
            {
                queue.push_back(s0);
            }
            if points_on_segment_r > 0
            {
                queue.push_back(s1);
            }
        }
        else 
        {
            queue.push_back(DCell{width : (left, right),height : (bottom, top), num_elems : n});
        }
        print!("\n \n");
    }

    LinkedList::from_iter(queue.into_iter().map(|DCell{width : (left, _),height : (bottom, _) ,num_elems : _}|{(left, bottom)}))
}
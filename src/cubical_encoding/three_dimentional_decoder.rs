use std::collections::LinkedList;
use std::fmt::Display;
use crate::def::bit_queue::BitQueue;
pub struct VCell{
    pub width : (u32,u32),
    pub height : (u32,u32),
    pub depth : (u32, u32),
    pub num_elems : usize
}
impl Display for VCell{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "width : {} - {}, height : {} - {}, depth {} - {}, num_of_points : {}\n", self.width.0, self.width.1, self.height.0, self.height.1,self.depth.0, self.depth.1, self.num_elems)
    }
}
fn is_any_cell_divisable(list : &LinkedList<VCell>) -> bool
{
    list.iter().any(|VCell{width : (left, right),height : (bottom, top), depth : (front, back), num_elems : _}|{ right - left > 0 || top - bottom > 0 || back - front > 0})
}
pub fn decode_cubical_segment(mut a : BitQueue, bounding_box : ((u32, u32), (u32, u32), (u32, u32))) -> LinkedList<(u32, u32, u32)>
{
    let mut queue : LinkedList<VCell> = LinkedList::new();
    queue.push_back(VCell{width : bounding_box.0, height : bounding_box.1, depth : bounding_box.2, num_elems : a.pop_front(32) as usize});

    while is_any_cell_divisable(&queue)
    {
        let VCell{width : (left, right), height : (bottom, top), depth : (front, back), num_elems : n} = queue.pop_front().unwrap();
        if right - left > 0 || top - bottom > 0 || back - front > 0
        {

            let points_on_segment_l = a.pop_front(n.ilog2() as usize + 1) as usize;
            let points_on_segment_r = n - points_on_segment_l;
            let s0 : VCell;
            let s1 : VCell;
            if right - left >= top - bottom && right - left >= back - front
            {
                s0 = VCell{width : (left, (right - left)/2 + left), height : (bottom, top), depth : (front, back), num_elems : points_on_segment_l};
                s1 = VCell{width : ((right - left)/2 + 1 + left, right), height : (bottom, top), depth : (front, back), num_elems : points_on_segment_r};   
            }
            else if top - bottom >= back - front
            {
                s0 = VCell{width : (left, right), height : (bottom, (top - bottom)/2 + bottom), depth : (front, back), num_elems : points_on_segment_l};
                s1 = VCell{width : (left, right), height : ((top - bottom)/2 + bottom + 1, top), depth : (front, back), num_elems : points_on_segment_r};
            }
            else
            {
                s0 = VCell{width : (left, right), height : (bottom, top), depth : (front, (back - front)/2 + front), num_elems : points_on_segment_l};
                s1 = VCell{width : (left, right), height : (bottom, top), depth : ((back - front)/2 + front + 1, back), num_elems : points_on_segment_r};
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
            queue.push_back(VCell{width : (left, right), height : (bottom, top), depth : (front, back), num_elems : n});
        }
    }

    LinkedList::from_iter(queue.into_iter().map(|VCell{width : (left, _),height : (bottom, _), depth : (front, _), num_elems : _}|{(left, bottom, front)}))
}
use core::fmt;
use std::collections::LinkedList;
use std::fmt::{write, Display};
struct DSegment
{
    size : (u32, u32),
    num_of_points : usize
}
impl fmt::Display for DSegment
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size : {} - {} number of points : {}", self.size.0, self. size.1, self.num_of_points)
    }
}
fn is_any_segemnt_divisable(list : &LinkedList<DSegment>) -> bool
{
    list.iter().any(|DSegment{size : (left, right), num_of_points : _}|{ right - left > 0 })
}
pub fn decode_line_segment(mut a : LinkedList<usize>, bounding_box : (u32,u32)) -> LinkedList<u32>
{
    let mut queue : LinkedList<DSegment> = LinkedList::new();
    queue.push_back(DSegment{size : bounding_box, num_of_points : a.pop_front().unwrap()});
    while is_any_segemnt_divisable(&queue)
    {
        let DSegment{size : (left, right), num_of_points : n} = queue.pop_front().unwrap();
        if right - left > 0
        {
            let points_on_segment_l = a.pop_front().unwrap();
            let points_on_segment_r = n - points_on_segment_l;
            let s0 = DSegment{ size : (left, (right - left)/2 + left), num_of_points : points_on_segment_l};
            let s1 = DSegment{ size : ((right - left)/2 + 1 + left, right), num_of_points : points_on_segment_r};
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
            queue.push_back(DSegment{size : (left, right), num_of_points : n});
        }
    }

    LinkedList::from_iter(queue.into_iter().map(|DSegment{size : (left, _), num_of_points : _}|{left}))
}
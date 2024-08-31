use std::collections::LinkedList;

pub struct Segment{
    pub size : (u32,u32),
    pub elems : LinkedList<u32>
}
fn segment_length(s : &Segment) -> usize
{
    let (left, right) = s.size;
    (right - left) as usize
}
pub fn encode_line_segment(line : Segment) -> LinkedList<usize>
{
    let mut encoding: LinkedList<usize> = LinkedList::new();
    encoding.push_front(line.elems.len());
    let mut queue: LinkedList<Segment> = LinkedList::new();
    queue.push_back(line);
    while queue.len() > 0
    {
        let current_line = queue.pop_front().unwrap();
        let (left, right) = current_line.size;
        let mut s0 = Segment{size : (left, (right - left)/2 + left), elems : LinkedList::new()};
        let mut s1 = Segment{size : ((right - left)/2 + 1 + left, right), elems : LinkedList::new()};
        for number in current_line.elems.iter()
        {
            if *number <= (right - left)/2 + left
            {
                s0.elems.push_front(*number)
            }
            else 
            {
                s1.elems.push_front(*number)    
            }
        }
        encoding.push_back(s0.elems.len());
        if s0.elems.len() > 0 && segment_length(&s0) > 0
        {
            queue.push_back(s0);
        }
        if s1.elems.len() > 0 && segment_length(&s1) > 0
        {
            queue.push_back(s1);
        }
    }
    encoding
}
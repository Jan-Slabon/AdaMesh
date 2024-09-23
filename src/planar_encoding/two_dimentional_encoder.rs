use std::collections::LinkedList;
use crate::def::bit_queue::BitQueue;
pub struct Cell{
    pub width : (u32,u32),
    pub height : (u32,u32),
    pub elems : LinkedList<(u32,u32)>
}
fn cell_area(s : &Cell) -> u32
{
    let (left, right) = s.width;
    let (bottom, top) = s.height;
    (right - left + 1) * (top - bottom + 1)
}
pub fn encode_planar_segment(cell : Cell) -> BitQueue
{
    let mut encoding: BitQueue = BitQueue{read_index : 0, write_index : 0, read_offset : 0, write_offset : 0, queue : Vec::from([])};
    encoding.push_back(cell.elems.len() as u32, 32);
    let mut queue: LinkedList<Cell> = LinkedList::new();
    queue.push_back(cell);
    while queue.len() > 0
    {
        let current_cell = queue.pop_front().unwrap();
        let (left, right) = current_cell.width;
        let (bottom, top) = current_cell.height;
        let mut s0 : Cell;
        let mut s1 : Cell;
        if right - left >= top - bottom
        {
            s0 = Cell{width : (left, (right - left)/2 + left), height : (bottom, top), elems : LinkedList::new()};
            s1 = Cell{width : ((right - left)/2 + 1 + left, right), height : (bottom, top), elems : LinkedList::new()};   
        }
        else
        {
            s0 = Cell{width : (left, right), height : (bottom, (top - bottom)/2 + bottom), elems : LinkedList::new()};
            s1 = Cell{width : (left, right), height : ((top - bottom)/2 + bottom + 1, top), elems : LinkedList::new()};
        }
        for (width, height) in current_cell.elems.iter()
        {
            if *width <= s0.width.1 && *height <= s0.height.1
            {
                s0.elems.push_front((*width, *height))
            }
            else
            {
                s1.elems.push_front((*width, *height)) 
            }
        }
        encoding.push_back(s0.elems.len() as u32, (current_cell.elems.len().ilog2() + 1) as usize);
        
        if s0.elems.len() > 0 && cell_area(&s0) > 1
        {
            queue.push_back(s0);
        }
        if s1.elems.len() > 0 && cell_area(&s1) > 1
        {
            queue.push_back(s1);
        }
    }
    encoding
}
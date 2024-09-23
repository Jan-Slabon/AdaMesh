use std::{collections::LinkedList, fmt::Display};
use crate::def::bit_queue::BitQueue;
pub struct Cell{
    pub width : (u32,u32),
    pub height : (u32,u32),
    pub depth : (u32, u32),
    pub elems : LinkedList<(u32,u32,u32)>
}
impl Display for Cell
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = write!(f, "width {} - {}, height {} - {}, depth {} - {}, number of points: {} ", self.width.0, self.width.1, self.height.0, self.height.1, self.depth.0, self.depth.1, self.elems.len());
        self.elems.iter().for_each(|(a,b,c)|print!("({}, {}, {})", a,b,c));
        println!();
        res
    }
}
fn cube_area(s : &Cell) -> u32
{
    let (left, right) = s.width;
    let (bottom, top) = s.height;
    let (front , back) = s.depth;
    (right - left + 1) * (top - bottom + 1) * (back - front + 1)
}
pub fn encode_cubical_segment(cell : Cell) -> BitQueue
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
        let (front, back) = current_cell.depth;
        let mut s0 : Cell;
        let mut s1 : Cell;
        if right - left >= top - bottom && right - left >= back - front
        {
            s0 = Cell{width : (left, (right - left)/2 + left), height : (bottom, top), depth : (front, back), elems : LinkedList::new()};
            s1 = Cell{width : ((right - left)/2 + 1 + left, right), height : (bottom, top), depth : (front, back), elems : LinkedList::new()};   
        }
        else if top - bottom >= back - front
        {
            s0 = Cell{width : (left, right), height : (bottom, (top - bottom)/2 + bottom), depth : (front, back), elems : LinkedList::new()};
            s1 = Cell{width : (left, right), height : ((top - bottom)/2 + bottom + 1, top), depth : (front, back), elems : LinkedList::new()};
        }
        else
        {
            s0 = Cell{width : (left, right), height : (bottom, top), depth : (front, (back - front)/ 2 + front), elems : LinkedList::new()};
            s1 = Cell{width : (left, right), height : (bottom, top), depth : ((back- front)/2 + front + 1, back), elems : LinkedList::new()};
        }
        for (width, height, depth) in current_cell.elems.iter()
        {
            if *width <= s0.width.1 && *height <= s0.height.1 && *depth <= s0.depth.1
            {
                s0.elems.push_front((*width, *height, *depth))
            }
            else
            {
                s1.elems.push_front((*width, *height, *depth)) 
            }
        }
        encoding.push_back(s0.elems.len() as u32, (current_cell.elems.len().ilog2() + 1) as usize);
        if s0.elems.len() > 0 && cube_area(&s0) > 1
        {
            queue.push_back(s0);
        }
        if s1.elems.len() > 0 && cube_area(&s1) > 1
        {
            queue.push_back(s1);
        }
    }
    encoding
}
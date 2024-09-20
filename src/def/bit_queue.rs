pub struct BitQueue{
    pub read_index : usize,
    pub write_index : usize,
    pub read_offset : u8,
    pub write_offset : u8,
    pub queue : Vec<u8>
}
impl BitQueue{
    pub fn push_back(&mut self, mut value : u32, mut size : usize)
    {
        assert!((value as u64) < (2 as u64).pow(size as u32));
        while self.write_offset as usize + size >= 8
        {
            let offset_value : u8 = (value as u8) << self.write_offset;
            size = size - (8 - self.write_offset as usize);
            if self.write_index >= self.queue.len() {self.queue.push(0);}
            self.queue[self.write_index] |= offset_value;
            self.write_index += 1;
            value = value >> (8 - self.write_offset as usize);
            self.write_offset = 0;
        }
        if size > 0
        {
            let offset_value : u8 = (value as u8) << self.write_offset;
            self.write_offset += size as u8;
            if self.write_index >= self.queue.len() {self.queue.push(0);}
            self.queue[self.write_index] |= offset_value;
        }
    }
    pub fn pop_front(&mut self, mut size : usize) -> u32
    {
        assert!(size <= 32);
        let mut result : u32 = 0;
        let mut bit_shift = 0;
        while self.read_offset as usize + size >= 8
        {
            let val = self.queue[self.read_index] >> self.read_offset;
            result |= (val as u32) << bit_shift;
            bit_shift += 8 - self.read_offset;
            size = size - (8 - self.read_offset as usize);
            self.read_offset = 0;
            self.read_index += 1;
        }
        if size > 0
        {
            let val = self.queue[self.read_index] >> self.read_offset;
            let mask = (2 as u8).pow(size as u32) - 1;
            self.read_offset += size as u8;
            result |= ((val & mask) as u32) << bit_shift;
        }
        result
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_push_back_small_integers()
    {
        let tmp : Vec<u8> = Vec::from([0]);
        let mut queue = BitQueue{read_index : 0, write_index : 0, read_offset : 0, write_offset : 0, queue : tmp};
        queue.push_back(3, 2);
        queue.push_back(3, 2);
        assert!(queue.queue[queue.write_index] == 15);
    }
    #[test]
    fn test_pop_front_small_integers()
    {
        let tmp : Vec<u8> = Vec::from([0b01100101]);
        let mut queue = BitQueue{read_index : 0, write_index : 0, read_offset : 0, write_offset : 0, queue : tmp};
        let first_read = queue.pop_front(3);
        let second_read = queue.pop_front(4);
        assert!(first_read == 5);
        assert!(second_read == 12);
    }
    #[test]
    fn test_push_back_large_integer()
    {
        let tmp : Vec<u8> = Vec::from([]);
        let mut queue = BitQueue{read_index : 0, write_index : 0, read_offset : 0, write_offset : 0, queue : tmp};
        queue.push_back(0b11001101101101101, 17);
        assert!(queue.queue[0] == 0b01101101);
        assert!(queue.queue[1] == 0b10011011);
        assert!(queue.queue[2] == 0b1);
    }
    #[test]
    fn test_pop_front_large_integer()
    {
        let tmp : Vec<u8> = Vec::from([0b01100101, 0b10110100]);
        let mut queue = BitQueue{read_index : 0, write_index : 0, read_offset : 0, write_offset : 0, queue : tmp};
        let read = queue.pop_front(16);
        assert!(read == 0b1011010001100101);
    }
    #[test]
    fn test_push_pop_interaction()
    {
        let tmp : Vec<u8> = Vec::from([]);
        let mut queue = BitQueue{read_index : 0, write_index : 0, read_offset : 0, write_offset : 0, queue : tmp};
        queue.push_back(5, 32);
        queue.push_back(10, 4);
        queue.push_back(12, 4);
        queue.push_back(1, 1);
        let first_read = queue.pop_front(32);
        let second_read = queue.pop_front(4);
        let third_read = queue.pop_front(4);
        let forth_read = queue.pop_front(1);
        assert!(first_read == 5);
        assert!(second_read == 10);
        assert!(third_read == 12);
        assert!(forth_read == 1);
    }
}
use std::collections::LinkedList;
use std::path::absolute;

use ply_rs::{parser};
use ply_rs::ply::{self, Property};
extern crate ply_rs;
fn property_to_float(prop : Property) -> Option<f32>
{
    match prop {
        Property::Float(f) => Some(f),
        _ => None
    }
}
fn exponent(number : &f32) -> (u32, u32)
{
    let bits = number.to_bits();
    let mantisa = bits & 0b0_00000000_11111111111111111111111; // isolating mantisa
    let mut exp = bits >> 23; // cutting out mantisa
    exp = exp & 0b011111111; // cutting out sign
    exp = exp - 127;
    (exp, mantisa)
}
pub fn read()
{
    let path = "C:/Users/jslabon/Desktop/mgr/AdaMesh/data/bunny/reconstruction/bun_zipper_res4.ply";
    let mut f = std::fs::File::open(path).unwrap();
    let p = parser::Parser::<ply::DefaultElement>::new();
    let mut result = p.read_ply(&mut f).unwrap();
    let payload = &result.payload["vertex"];
    let mut points = LinkedList::from([]);
    payload.iter().for_each(|a|{points.push_back(
        ( property_to_float(a["x"].clone()).unwrap(), property_to_float(a["y"].clone()).unwrap(),
        property_to_float(a["z"].clone()).unwrap() ) ); });
    let min_iter : Vec<(f32,f32,f32)>= points.iter().map(|(f1,f2,f3)|{(f32::abs(*f1), f32::abs(*f2), f32::abs(*f3))}).collect();
    let (min_x, _, _) = min_iter.iter().min_by(|(x, _, _), (x2, _, _)|{(*x).total_cmp(x2)}).unwrap();
    let (_, min_y, _) = min_iter.iter().min_by(|(_, y, _), (_, y2, _)|{(*y).total_cmp(y2)}).unwrap();
    let (_, _, min_z) = min_iter.iter().min_by(|(_, _, z), (_, _, z2)|{(*z).total_cmp(z2)}).unwrap();

    points = points.iter().map(|(x,y,z)|{(x - min_x,y - min_y,z - min_z)}).collect() ;

}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_exponent_mantisa_extraction()
    {
        let float = f32::from_bits(0b0_10110100_01101110000111001011011);
        let result = exponent(&float);
        assert_eq!(result.0, 0b10110100 - 0b01111111);
        assert_eq!(result.1, 0b01101110000111001011011);
    }
}
use telperion_core::{mesh::{self, Detail}, params};
fn main() {
 let value=serde_json::from_str(&std::fs::read_to_string(".flow/evidence/fn31/ordinary-growth.json").unwrap()).unwrap();
 for floor in [0.0,0.75] {
  let mut f=params::parse(&value).unwrap(); f.growth.vigour_floor=floor;
  let m=mesh::build(&f,Detail::Full).unwrap();
  println!("original set 6 vigour_floor={floor} triangles={} foliage={}",m.wood_triangles(),m.foliage_instances());
 }
}

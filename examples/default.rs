extern crate swde;

fn main() {
    let s = swde::State::from(vec![
        swde::action::ConstructBuilding::new(10),
        swde::action::ConstructBuilding::new(7),
    ]).unwrap();

    println!("{:?}", s)
}
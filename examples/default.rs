extern crate swde;

fn main() {
    let state = swde::State::from(vec![
        swde::action::ConstructBuilding::new(10),
        swde::action::ConstructBuilding::new(7),
    ]);

    match state {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{:?}", e)
    }
}
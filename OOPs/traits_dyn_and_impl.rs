trait LandCapable {
    fn drive(&self) {
        println!("Vehicle is driving");
    }
}

struct Sedan;
impl LandCapable for Sedan {
    fn drive(&self) {
        println!("Sedan is driving!");
    }
}

struct Suv;
impl LandCapable for Suv {
    fn drive(&self) {
        println!("SUV is driving!!");
    }
}

fn roadtrip(vehicle: &dyn LandCapable) {
    vehicle.drive();
}
//--------------------------------------------------------------------
trait WaterCapable {
    fn float(&self) {
        println!("default floating..");
    }
}
trait Amphibious: WaterCapable + LandCapable {
    fn hower(&self) {
        println!("I am howering over the surface");
    }
}
struct Hovercraft;
impl Amphibious for Hovercraft {}
impl LandCapable for Hovercraft {}
impl WaterCapable for Hovercraft {}
fn traverse_frozen_lake(vehicle: &impl Amphibious) {
    vehicle.hower();
    vehicle.drive();
    vehicle.float();
}

fn main() {
    println!("Hello world");
    let car = Sedan;
    let jeep = Suv;
    roadtrip(&car);
    roadtrip(&jeep);
    let hc = Hovercraft;
    traverse_frozen_lake(&hc);
}

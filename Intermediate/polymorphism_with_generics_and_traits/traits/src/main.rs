struct Truck {
    info: VehicleInfo,
}

struct Car {
    info: VehicleInfo,
}

struct House {}

impl Paint for House {
    fn paint(&self, color: &str) {
        println!("painting house {color}");
    }
}

trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: &str) {
        println!("painting object {color}");
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: String,
}

impl Park for Car {
    fn park(&self) {
        println!("Parking car...")
    }
}

impl Park for Truck {
    fn park(&self) {
        println!("Parking truck...")
    }
}

impl Paint for Car {}

impl Paint for Truck {}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "VM".to_owned(),
            model: "Volvo".to_owned(),
            year: "2015".to_owned(),
        },
    };

    car.paint("blue");
    car.park();
}


trait Vehicle: Paint { // Qualquer tipo implementando Vehicle, precisa implementar Paint
    fn park(&self);
}

/**
 * 
trait Vehicle: Paint + Park { -> Uma trait pode ter varias super traits... Paint e Park são supertraits de Vehicle
}
 */



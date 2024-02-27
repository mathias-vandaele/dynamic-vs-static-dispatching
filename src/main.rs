use std::hint::black_box;
use std::time::Instant;

fn main() {
    let m = Mercedes { hp: 234 };
    let a = AMG { hp: 987 };

    let start1 = Instant::now();
    for i in 0..100000000 {
        let v = test_dynamic_dispatching(&a, &m);
        black_box(v.get_hp());
    }
    let duration1 = start1.elapsed();
    println!("Time elapsed in dynamic dispatching is: {:?}", duration1);


    let start2 = Instant::now();
    for i in 0..100000000 {
        let v = test_static_dispatching(&a, &m);
        match v {
            VehicleDispatcher::AMG(amg) => { black_box(amg.get_hp()); }
            VehicleDispatcher::Mercedes(mer) => { black_box(mer.get_hp()); }
        };
    }
    let duration2 = start2.elapsed();
    println!("Time elapsed in static dispatching is: {:?}", duration2);
}

trait Vehicle {
    fn get_hp(&self) -> u32;
}

struct Mercedes {
    hp: u32,
}

struct AMG {
    hp: u32,
}

impl Vehicle for Mercedes {
    fn get_hp(&self) -> u32 {
        self.hp
    }
}

impl Vehicle for AMG {
    fn get_hp(&self) -> u32 {
        self.hp * 2
    }
}


enum VehicleDispatcher<'a> {
    AMG(&'a AMG),
    Mercedes(&'a Mercedes),
}

fn test_dynamic_dispatching<'a>(x: &'a AMG, y: &'a Mercedes) -> &'a dyn Vehicle {
    if x.get_hp() > y.get_hp() {
        return x;
    }
    y
}

fn test_static_dispatching<'a>(x: &'a AMG, y: &'a Mercedes) -> VehicleDispatcher<'a> {
    if x.get_hp() > y.get_hp() {
        return VehicleDispatcher::AMG(&x);
    }
    VehicleDispatcher::Mercedes(&y)
}
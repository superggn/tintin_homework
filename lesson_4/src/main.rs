struct People {
    name: String,
}
impl People {
    fn hello(&self) {
        println!("hello, {}!", self.name);
    }
}
struct Car {
    name: String,
}
impl Car {
    fn price() {
        println!("29.9");
    }
}

struct Resident {
    name: String,
}
impl Resident {
    fn residential_type(&self) {
        println!("name: {}", self.name);
    }
}

enum MyEnum {
    People(People),
    Vehicle(Car),
    LivePlace(Resident),
}

trait MyTrait {
    fn hello(&self);
}

impl MyTrait for Resident {
    fn hello(&self) {
        println!("hello, {}!", self.name);
    }
}

/// =================================================================
use std::ops::Add;

#[derive(Debug)]
struct WaterTank {
    volume: u32,
}
impl Add for WaterTank {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            volume: self.volume + other.volume,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut data_list: Vec<MyEnum> = Vec::new();
    data_list.push(MyEnum::People(People {
        name: "xiaoming".to_string(),
    }));
    for item in data_list.iter() {
        if let MyEnum::People(people) = item {
            people.hello();
        }
    }
    let mut trait_list: Vec<Box<dyn MyTrait>> = Vec::new();
    trait_list.push(Box::new(Resident {
        name: "怨灵古堡".to_string(),
    }));
    for item in trait_list.iter() {
        item.hello();
    }
    // =================================================================
    let my_water_tank = WaterTank { volume: 100 };
    let xiaoming_water_tank = WaterTank { volume: 20 };
    fn tank_add<T: Add<Output = T> + 'static>(left: Box<T>, right: Box<T>) -> Box<T> {
        Box::new(*left + *right)
    }
    let res = tank_add(Box::new(my_water_tank), Box::new(xiaoming_water_tank));
    println!("res: {:?}", res);
}

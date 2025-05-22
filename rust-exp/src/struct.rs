// also call class

fn main() {
    let instance = Birth {
        name: String::from("Bale"),
        attack: 100,
    };
    
    instance.get_name();

    println!("Can fly: {}", instance.can_fly());
    println!("Can swim: {}", instance.can_swim());
}


struct Birth {
    name: String,
    attack: u64,
}

impl Birth {
    fn get_name(&self) {
        println!("{}", self.name);
    }
}

impl Animal for Birth {
    fn can_fly(&self) -> bool {
        true
    }

    fn can_swim(&self) -> bool {
        true
    }
}

trait Animal {
    fn can_fly(&self) -> bool;
    fn can_swim(&self) -> bool {
        false
    }
}
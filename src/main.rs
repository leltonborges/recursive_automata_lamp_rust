use std::rc::Rc;
use std::cell::RefCell;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct Lamp {
    name: &'static str,
    on: bool,
    prev: Option<Rc<RefCell<Lamp>>>,
    next: Option<Rc<RefCell<Lamp>>>,
}

impl Lamp {
    fn new(name: &'static str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { name, on: false, prev: None, next: None }))
    }

    fn turn_on(&mut self) {
        self.on = true;
        println!("{} is now on.", self.name);
        thread::sleep(Duration::from_secs(1));
    }

    fn turn_off(&mut self) {
        self.on = false;
        println!("{} is now off.", self.name);
        thread::sleep(Duration::from_secs(1));
    }

    fn operate_exchange<F>(&mut self, operation: F)
    where F: Fn(&mut Self) + Copy {
        operation(self);
        operate_exchange_prev_next(&self.prev, &self.next, operation);
    }
}

fn link_with(lamp: &Rc<RefCell<Lamp>>, other: Rc<RefCell<Lamp>>) {
    lamp.borrow_mut().next = Some(other.clone());
    other.borrow_mut().prev = Some(lamp.clone());
}

fn operate_exchange_prev_next<F>(prev: &Option<Rc<RefCell<Lamp>>>,
                                 next: &Option<Rc<RefCell<Lamp>>>,
                                 operation: F)
where F: Fn(&mut Lamp) + Copy {
    if let Some(ref prev_lamp) = prev {
        operation(&mut prev_lamp.borrow_mut());
    }

    if let Some(ref next_lamp) = next {
        operation(&mut next_lamp.borrow_mut());
    }

    let prev_prev_lamp = prev.as_ref()
                             .and_then(|lamp| lamp.borrow().prev.clone());
    let next_next_lamp = next.as_ref()
                             .and_then(|lamp| lamp.borrow().next.clone());

    if prev_prev_lamp.is_none() && next_next_lamp.is_none() {
        return;
    }

    operate_exchange_prev_next(&prev_prev_lamp, &next_next_lamp, operation);
}

fn setup_chain() -> Rc<RefCell<Lamp>> {
    let lamps = [
        Lamp::new("Lamp A"),
        Lamp::new("Lamp B"),
        Lamp::new("Lamp C"),
        Lamp::new("Lamp D"),
        Lamp::new("Lamp E"),
        Lamp::new("Lamp F"),
        Lamp::new("Lamp G"),
        Lamp::new("Lamp H"),
        Lamp::new("Lamp I"),
    ];

    for window in lamps.windows(2) {
        link_with(&window[0], window[1].clone());
    }

    lamps[lamps.len() - 1].clone()
}

fn main() {
    let start_lamp = setup_chain();

    let operation_torn_on = |lamp: &mut Lamp| {
        if !lamp.on {
            lamp.turn_on()
        }
    };

    let operation_torn_off = |lamp: &mut Lamp| {
        if lamp.on {
            lamp.turn_off()
        }
    };

    println!("***************************");
    println!("********** START **********");
    println!("***************************");

    println!();
    println!("********** ON ***********");
    start_lamp.borrow_mut().operate_exchange(operation_torn_on);
    println!("********* END ON ********");

    println!();
    println!("********** OFF **********");
    start_lamp.borrow_mut().operate_exchange(operation_torn_off);
    println!("******** END OFF ********");

    println!();
    println!("**************************");
    println!("********** END ***********");
    println!("**************************");
}

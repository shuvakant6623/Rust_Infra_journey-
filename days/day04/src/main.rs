use std::fmt::Debug;

// Trait Definition 

trait Engine {
    fn name(&self) -> &str;
    fn run(&self);
}

// Implementation

struct BatchEngine;
struct StreamEngine;

impl Engine for BatchEngine {
    fn name(&self) -> &str {
        "Batch Engine"
    }

    fn run(&self) {
        println!("Running batch jobs...");
    }
}

impl Engine for StreamEngine {
    fn name(&self) -> &str {
        "Stream Engine"
    }

    fn run(&self) {
        println!("Processing stream data...");
    }
}

// Generic Function 

fn start_engine<T: Engine>(engine: T) {
    println!("Starting: {}", engine.name());
    engine.run();
}

// Trait Objects (Runtime Polymorphism) 

fn start_dyn(engine: Box<dyn Engine>) {
    println!("Starting dynamically: {}", engine.name());
    engine.run();
}

// Generic Struct

struct Executor<T: Engine> {
    engine: T,
}

impl<T: Engine> Executor<T> {
    fn execute(&self) {
        println!("Executor using: {}", self.engine.name());
        self.engine.run();
    }
}

// Generic Utility

fn log_value<T: Debug>(val: T) {
    println!("Debug value: {:?}", val);
}

// main

fn main() {
    println!("---Day 4: Traits & Generics---");

    let batch = BatchEngine;
    let stream = StreamEngine;

    start_engine(batch);
    start_engine(stream);

    let dyn_engine: Box<dyn Engine> = Box::new(BatchEngine);
    start_dyn(dyn_engine);

    let exec = Executor {
        engine: StreamEngine,
    };

    exec.execute();

    log_value(100);
    log_value("infra-system");

    println!("---End of day 4---");

}
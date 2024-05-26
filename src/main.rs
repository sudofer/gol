use std::any::{Any, TypeId};
use std::io;

mod examples {
    use std::any::type_name_of_val;

    pub fn iterate_modules() {
        let module_names: Vec<&str> =
            type_name_of_val(&crate::examples::char_callback::char_callback)
                .split("::")
                .skip(2)
                .collect();

        for module_name in module_names {
            if module_name != "char_callback" {
                println!("Found module: {}", module_name);
            }
        }
    }

    pub mod char_callback;
    pub mod fractal;
    pub mod white_noise;
}

fn main() {
    examples::iterate_modules();
    let example_modules: Vec<&'static str> = vec![
        "examples::char_callback",
        "examples::fractal",
        "examples::white_noise",
    ];

    println!("Available examples:");

    for (i, module) in example_modules.iter().enumerate() {
        println!("{}. {}", i + 1, module);
    }

    println!("Enter the number of the example to run:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(error) => {
            println!("look at what you've done: {error}");
            return;
        }
    };

    if input > 0 && input <= example_modules.len() {
        let module_name = example_modules[input - 1];
        run_example(module_name);
    } else {
        println!("Invalid choice");
    }

    fn run_example(module_name: &'static str) {
        match module_name {
            "examples::fractal" => examples::fractal::fractal(),
            "examples::white_noise" => examples::white_noise::white_noise(),
            "examples::char_callback" => examples::char_callback::char_callback(),
            _ => println!("Example not found"),
        }
    }
}

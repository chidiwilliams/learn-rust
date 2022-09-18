use std::collections::binary_heap::Iter;

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    fn new(name: &'static str) -> Self
    where
        Self: Sized;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Sheep {
            name: name,
            naked: false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaah?"
        } else {
            "baaaaaah!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

struct Cow {
    name: &'static str,
}

impl Animal for Cow {
    fn new(name: &'static str) -> Self {
        Cow { name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "mooooooooooooo"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {
            name: "Dolly",
            naked: false,
        })
    } else {
        Box::new(Cow { name: "Sally" })
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");
    // let mut dolly2 = Sheep::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            line.map(|line| {
                line.split(',')
                    .map(|entry| String::from(entry.trim()))
                    .collect()
            })
        })
        .collect()
}

fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| x + y
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}

trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}.",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

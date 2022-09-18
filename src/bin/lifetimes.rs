fn main() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);

    let mut owner = Owner(18);
    owner.add_one();
    owner.print();

    let first = 2;

    {
        let second = 3;
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }

    {
        let static_string = "I'm in read-only memory";
        println!("static string: {}", static_string);
    }

    {}
}

fn print_refs<'a, 'b>(x: &i32, y: &i32) {
    println!("x is {}, y is {}", x, y);
}

struct Owner(i32);

impl Owner {
    fn add_one(&mut self) {
        self.0 += 1;
    }

    fn print(&self) {
        println!("`print`: {}", self.0);
    }
}

fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &i32 {
    &NUM
}

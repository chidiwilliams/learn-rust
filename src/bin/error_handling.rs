use std::num::ParseIntError;

enum Food {
    Apple,
    Carrot,
    Potato,
}

struct Peeled(Food);
struct Chopped(Food);
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Chopped> {
    match food {
        Some(food) => Some(Chopped(food)),
        None => None,
    }
}

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let number = vec!["41", "93", "18"];
    println!("The first doubled is {:?}", double_first(number));

    other_uses::main();
    wrapping_errors::main();
}

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number: i32 = first_number_str.parse().unwrap();
    let second_number: i32 = second_number_str.parse().unwrap();
    first_number * second_number
}

fn multiply_result(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn multiply_early_returns(
    first_number_str: &str,
    second_number_str: &str,
) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));
    opt.map_or(Ok(None), |r| r.map(Some))
}

mod other_uses {
    use std::{error, fmt, vec};

    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive(Debug)]
    struct EmptyVec;

    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    impl error::Error for EmptyVec {}

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        let first = vec.first().ok_or(EmptyVec)?;
        let parsed = first.parse::<i32>()?;
        Ok(2 * parsed)
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubld is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub(crate) fn main() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}

mod wrapping_errors {
    use std::error::Error;
    use std::{error, fmt, num::ParseIntError};

    type Result<T> = std::result::Result<T, DoubleError>;

    #[derive(Debug)]
    enum DoubleError {
        EmptyVec,
        Parse(ParseIntError),
    }

    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
                DoubleError::Parse(..) => {
                    write!(f, "the provided string could not be parsed as int")
                }
            }
        }
    }

    impl error::Error for DoubleError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match *self {
                DoubleError::EmptyVec => None,
                DoubleError::Parse(ref e) => Some(e),
            }
        }
    }

    impl From<ParseIntError> for DoubleError {
        fn from(err: ParseIntError) -> Self {
            DoubleError::Parse(err)
        }
    }

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        let first = vec.first().ok_or(DoubleError::EmptyVec)?;
        let parsed = first.parse::<i32>()?;
        Ok(2 * parsed)
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => {
                println!("Error: {}", e);
                if let Some(source) = e.source() {
                    println!(" Caused by: {}", source)
                }
            }
        }
    }

    pub(crate) fn main() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}

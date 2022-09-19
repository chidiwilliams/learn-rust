use std::{thread, vec};

const THREAD_COUNT: usize = 10;

fn main() {
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    // let mut children = vec![];

    let chunked_data = data.split_whitespace();

    const TASK: Vec<&str> = vec![];
    let mut tasks = [TASK; THREAD_COUNT];

    chunked_data.enumerate().for_each(|(i, str)| {
        tasks[i % THREAD_COUNT].push(str);
    });

    let final_result = tasks
        .into_iter()
        .map(|task| {
            thread::spawn(move || {
                task.into_iter()
                    .map(|line| {
                        line.chars()
                            .map(|c| c.to_digit(10).expect("should be a digit"))
                            .sum::<u32>()
                    })
                    .sum::<u32>()
            })
        })
        .map(|c| c.join().unwrap())
        .sum::<u32>();

    // for (i, data_segment) in chunked_data.enumerate() {
    //     println!("data segment {} is \"{}\"", i, data_segment);

    //     children.push(thread::spawn(move || -> u32 {
    //         let result = data_segment
    //             .chars()
    //             .map(|c| c.to_digit(10).expect("should be a digit"))
    //             .sum();
    //         println!("processed segment {}, result={}", i, result);
    //         result
    //     }))
    // }

    // let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();
    println!("Final sum result: {}", final_result);
}

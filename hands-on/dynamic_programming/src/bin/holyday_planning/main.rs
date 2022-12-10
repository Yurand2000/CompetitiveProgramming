mod input_parse;

fn main()
{
    let input = include_str!("./tests/input0.txt");
    let input_lines = &mut input.lines().map(|line| String::from(line));
    let (mut cities, days) = input_parse::read_input(input_lines);

    for city in cities.iter_mut() {
        prefix_sum(city);
    }

    solve(cities, days as usize);
}

fn prefix_sum(vec: &mut Vec<i32>) {
    let mut sum = 0;
    for curr in vec.iter_mut() {
        sum += *curr;
        *curr = sum;
    }
}

fn solve(cities: Vec<Vec<i32>>, days: usize)
{
    let mut table: Vec<Vec<i32>> = Vec::with_capacity(cities.len());
    for _ in 0..cities.len() { table.push(Vec::new()); }

    //initialize city 0
    table[0].push(0);
    for day in 1..=days {
        table[0].push(cities[0][day - 1]);
    }

    for city in 1..cities.len() {
        for day in 0..=days
        {
            let mut max = table[city - 1][day];
            for prec_day in 0..day {
                let curr = table[city - 1][prec_day] + cities[city][day - prec_day - 1];

                if curr > max {
                    max = curr;
                }
            }

            table[city].push(max);
        }
    }

    for city in 0..cities.len() {
        for day in 0..=days {
            print!("{:4}", table[city][day]);
        }
        println!("");
    }
}
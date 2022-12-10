use std::ops::Index;

mod input_parse;

pub struct City {
    pub attractions: Vec<i32>,
}

pub struct Table {
    table: Vec<i32>,
    row_size: usize,
}

fn main()
{
    let (mut cities, days) = input_parse::read_input_from_stdin();

    preprocessing(&mut cities);
    let max_attractions = solve(cities, days);

    println!("{}", max_attractions);
}

fn preprocessing(cities: &mut Vec<City>) {
    for city in cities.iter_mut() {
        prefix_sum(city);
    }
}

fn prefix_sum(city: &mut City) {
    let mut sum = 0;
    for curr in city.attractions.iter_mut() {
        sum += *curr;
        *curr = sum;
    }
}

fn solve(cities: Vec<City>, days: usize) -> i32
{
    let mut table = Table::new(cities.len(), days + 1);

    //initialize city 0
    table[(0, 0)] = 0;
    for day in 1..=days {
        table[(0, day)] = cities[0].attractions[day - 1];
    }

    //include cities one by one
    for city in 1..cities.len() {
        for day in 0..=days
        {
            let mut max = table[(city - 1, day)];
            for prec_day in 0..day {
                let curr = table[(city - 1, prec_day)] + cities[city].attractions[day - prec_day - 1];

                if curr > max {
                    max = curr;
                }
            }

            table[(city, day)] = max;
        }
    }

    table[(cities.len() - 1, days)]
}

impl City {
    pub fn new() -> City {
        City { attractions: Vec::new() }
    }
}

impl Table {
    pub fn new(rows: usize, columns: usize) -> Self {
        Table { table: vec![0; rows * columns], row_size: columns }
    }

    fn __index(&self, row: usize, column: usize) -> usize {
        self.row_size * row + column
    }
}

impl Index<(usize, usize)> for Table {
    type Output = i32;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        self.table.index( self.__index(row, column) )
    }
}

impl std::ops::IndexMut<(usize, usize)> for Table {

    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        self.table.index_mut( self.__index(row, column) )
    }
}

#[cfg(test)]
mod tests;
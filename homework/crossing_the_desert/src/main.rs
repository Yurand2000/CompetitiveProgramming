

struct Station {
    pub fuel_provided: i32,
    pub next_station_distance: i32
}

struct Solution;

impl Solution {
    pub fn minimum_fuel(stations: &[Station]) -> i32
    {
        let rec = Solution::minimum_fuel_recursive(stations);
        let lin = Solution::minimum_fuel_linear(stations);

        assert_eq!(rec, lin);
        rec
    }

    pub fn minimum_fuel_linear(stations: &[Station]) -> i32
    {
        let mut min_fuel = 0;
        for station in stations.iter().rev()
        {
            min_fuel += station.next_station_distance;
            min_fuel = std::cmp::max(0, min_fuel - station.fuel_provided);
        }

        min_fuel
    }

    pub fn minimum_fuel_recursive(stations: &[Station]) -> i32
    {
        Solution::__minimum_fuel_recursive(stations).0
    }

    pub fn __minimum_fuel_recursive(stations: &[Station]) -> (i32, i32)
    {
        if stations.len() > 1
        {
            let (left, right) = stations.split_at( stations.len() / 2 );

            let min_left = Solution::__minimum_fuel_recursive(left);
            let min_right = Solution::__minimum_fuel_recursive(right);

            let fuel_needed = min_left.0 - min_left.1 + min_right.0;
            let fuel_provided = min_right.1 - fuel_needed;
            ( std::cmp::max(0, fuel_needed), std::cmp::max(0, fuel_provided) )
        }
        else
        {
            let fuel_needed = stations[0].next_station_distance;
            let fuel_provided = stations[0].fuel_provided;

            ( std::cmp::max(0, fuel_needed - fuel_provided), std::cmp::max(0, fuel_provided - fuel_needed) )
        }
    }
}

mod tests;

fn main()
{
    let input = [Station::distance(5), Station::new(1, 2), Station::new(6, 5), Station::new(4, 9), Station::new(9, 4)];
    let output = 10;

    assert_eq!(Solution::minimum_fuel(&input), output);
}

impl Station
{
    pub fn new(fuel_provided: i32, next_station_distance: i32) -> Station {
        assert!(fuel_provided >= 0);
        assert!(next_station_distance >= 0);
        Station { fuel_provided, next_station_distance }
    }

    pub fn distance(next_station_distance: i32) -> Station {
        assert!(next_station_distance >= 0);
        Station { fuel_provided: 0, next_station_distance }
    }
}

#[cfg(test)]
mod tests
{
    use crate::{Station, Solution};

    #[test]
    fn no_stations_min_fuel()
    {
        let input = [Station::distance(5)];
        let output = 5;

        assert_eq!( Solution::minimum_fuel(&input), output );
    }

    #[test]
    fn more_fuel_than_necessary()
    {
        let input = [Station::new(7, 5)];
        let output = 0;

        assert_eq!( Solution::minimum_fuel(&input), output );
    }

    #[test]
    fn one_station_with_not_enough_fuel()
    {
        let input = [Station::distance(0), Station::new(2, 3)];
        let output = 1;

        assert_eq!( Solution::minimum_fuel(&input), output );
    }
    
    #[test]
    fn one_station_with_more_fuel()
    {
        let input = [Station::distance(0), Station::new(5, 3)];
        let output = 0;

        assert_eq!( Solution::minimum_fuel(&input), output );
    }

    #[test]
    fn more_fuel_than_necessary_one_station()
    {
        let input = [Station::new(100, 5), Station::new(7, 5)];
        let output = 0;

        assert_eq!( Solution::minimum_fuel(&input), output );
    }

    #[test]
    fn one_station_with_more_fuel_2()
    {
        let input = [Station::distance(7), Station::new(5, 3)];
        let output = 7;

        assert_eq!( Solution::minimum_fuel(&input), output );
    }

    #[test]
    fn two_stations_with_not_enough_fuel()
    {
        let input = [Station::distance(2), Station::new(1, 3), Station::new(2, 3)];
        let output = 5;

        assert_eq!( Solution::minimum_fuel(&input), output );
    }

    #[test]
    fn two_stations_with_more_fuel()
    {
        let input = [Station::distance(2), Station::new(6, 3), Station::new(7, 3)];
        let output = 2;

        assert_eq!( Solution::minimum_fuel(&input), output );
    }

    #[test]
    fn two_stations_the_first_with_more_fuel()
    {
        let input = [Station::distance(2), Station::new(6, 3), Station::new(2, 3)];
        let output = 2;

        assert_eq!( Solution::minimum_fuel(&input), output );
    }

    #[test]
    fn two_stations_the_second_with_more_fuel()
    {
        let input = [Station::distance(2), Station::new(1, 3), Station::new(5, 3)];
        let output = 4;

        assert_eq!( Solution::minimum_fuel(&input), output );
    }

    #[test]
    fn more_fuel_than_necessary_two_stations()
    {
        let input = [Station::new(100, 5), Station::new(7, 5), Station::new(7, 5)];
        let output = 0;

        assert_eq!( Solution::minimum_fuel(&input), output );
    }

    #[test]
    fn unnamed_test()
    {
        let input = [Station::new(8, 5), Station::new(6, 5), Station::new(2, 5), Station::new(3, 5)];
        let output = 1;

        assert_eq!( Solution::minimum_fuel(&input), output );
    }

    #[test]
    fn bigger_input()
    {
        let input = [Station::distance(5), Station::new(1, 2), Station::new(6, 5), Station::new(4, 9), Station::new(9, 4)];
        let output = 10;

        assert_eq!( Solution::minimum_fuel(&input), output );
    }
}
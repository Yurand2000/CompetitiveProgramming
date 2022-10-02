// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

struct Solution
{
    bad: i32
}

impl Solution {
    fn isBadVersion(&self, n: i32) -> bool {
        n >= self.bad
    }

    pub fn first_bad_version(&self, n: i32) -> i32
    {
		self.bin_search(1, n)
    }
    
    fn bin_search(&self, start: i32, end: i32) -> i32
    {
        let middle = (end - start) / 2 + start;
        
        if start == end {
            middle
        }
        else if self.isBadVersion(middle)
        {
            self.bin_search(start, middle)
        }
        else
        {
            self.bin_search(middle+1, end)
        }
    }
}

fn main()
{
    let n = 2126753390;
    let bad = 1702766719;    

    println!("{}", Solution{bad}.first_bad_version(n) == bad);
}

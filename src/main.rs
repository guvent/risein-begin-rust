fn main() {

    let collect: Vec<i32> = vec![4,5,6,7,8];

    let filter: FilterCondition = FilterCondition{
        filtering: vec![1,2,3,4,5,6,7,8],
    };

    let has_found = filter.is_match(4);
    let not_found = filter.is_match(14);

    println!("Has Found ({}): {}", 4, has_found);
    println!("Not Found ({}): {}", 14, not_found);


    let filtered = filter.custom_filter(collect);

    println!("{:?}", filtered);


}

struct FilterCondition {
    filtering: Vec<i32>
}

impl FilterCondition {
    fn is_match(&self, x: i32) -> bool {
        let mut result = false;
        let filtering = &self.filtering;

        for item in filtering {
            if x == *item {
                result = true;
            }
        }

        result
    }

    fn custom_filter(&self, xs: Vec<i32>) -> Vec<i32> {
        let mut filtered = Vec::new();
        let filtering = &self.filtering;

        for item in filtering {
            for x in &xs {
                if *x == *item {
                    filtered.push(*x)
                }
            }
        }

        filtered
    }
}
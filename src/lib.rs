
pub fn execute<T: Clone + Send + Sync>(v: Vec<T>, f: fn(t: Vec<T>) -> T) -> Vec<T> {
    const THRESHOLD: usize = 10;

    if v.len() <= THRESHOLD {
        println!("executing in single thread...");

        let mut result = Vec::new();
        result.push(f(v));

        return result;
    }

    println!("executing in multi thread...");

    let mid = v.len() / 2;
    let (left, right) = v.split_at(mid);

    let mut result = Vec::new();

    crossbeam::scope(|s| {
        let handler_1 = s.spawn(|_| {
            f(left.to_vec())
        });
        
        let handler_2 = s.spawn(|_| {
            f(right.to_vec())
        });
    
        let mut items = Vec::new();
        items.push(handler_1.join().unwrap());
        items.push(handler_2.join().unwrap());

        result.push(f(items));

    }).unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_multi_thread() {
        let v = vec![1, 5, 3, 9, 7, 12, 4, 6, 0, -1, 14, 18, 44, 50];
        let res = execute(v, find_max);
        assert_eq!(vec![50], res);
    }
    
    #[test]
    fn find_max_single_thread() {
        let v = vec![1, 5, 3, 9, 7];
        let res = execute(v, find_max);
        assert_eq!(vec![9], res);
    }

    #[test]
    fn sum_multi_thread() {
        let v = vec![1, 5, 3, 9, 7, 1, 5, 3, 9, 7, 1, 5, 3, 9, 7, 1, 5, 3, 9, 7];
        let res = execute(v, sum);
        assert_eq!(vec![100], res);
    }
    
    #[test]
    fn sum_single_thread() {
        let v = vec![1, 5, 3, 9, 7];
        let res = execute(v, sum);
        assert_eq!(vec![25], res);
    }

    fn find_max(v: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        for item in v {
            if item > max {
                max = item;
            }
        }

        max
    }

    fn sum(v: Vec<i32>) -> i32 {
        let mut sum = 0;
    
        for item in v {
            sum += item;
        }
    
        sum
    }

}

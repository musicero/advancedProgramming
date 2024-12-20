use std::any::type_name_of_val;

// Golang:
// func (int) int
// 
// func callback(f func(int) int, i int) int {
//     return f(i)
//}

fn my_function(x: i32) -> i32 {
    x + 1
}


/*fn my_higher_order_function<F: Fn(i32)->i32>(f: F, i: i32) -> i32 {
    f(i)
}
fn my_higher_order_function<F>(f: F) -> i32
    where F: Fn(i32) -> i32 
{
    f(i)
}
*/
fn my_higher_order_function(f: fn(i32) -> i32, i: i32) -> i32 {
    f(i)
}

struct A(i32);

fn main() {
    let f = my_function; 
    // memory::my_function
    let g: fn(i32) -> i32 = my_function;


    //let my_closure: fn(i32) -> i32 = |x| x + 1;
    let b = A(10);
    let my_closure = || { b };

    let my_fn1: fn(i32) -> i32 = my_function;
    let my_fn2 = my_function;

    println!("my_closure: {}", type_name_of_val(&my_closure));
    println!("my_fn1: {}", type_name_of_val(&my_fn1));
    println!("my_fn2: {}", type_name_of_val(&my_fn2));
/* 
    println!("my_higher_order_function(my_closure, 1): {}", 
        my_higher_order_function(my_closure, 1));
*/
    println!("my_higher_order_function(my_fn1, 1): {}",
        my_higher_order_function(my_fn1, 1));

    println!("my_higher_order_function(my_fn2, 1): {}",
        my_higher_order_function(my_fn2, 1));
}


// tests
#[cfg(test)]
mod tests {
    use super::my_function;
    use std::any::type_name_of_val;

    #[test]
    fn test_coercion() {
        let my_fn: fn(i32) -> i32 = my_function;
        let my_closure: fn(i32) -> i32 = |x| x + 1;
        assert_eq!(type_name_of_val(&my_closure), type_name_of_val(&my_fn));
    }

    #[test]
    fn test_raw_type() {
        let my_fn = my_function;
        println!("my_fn: {}", type_name_of_val(&my_fn));
        assert_eq!(type_name_of_val(&my_fn), type_name_of_val(&my_function));
    }

    #[test]
    fn test_raw_type_and_coerced() {
        let my_fn1 = my_function;
        let my_fn2: fn(i32) -> i32 = my_function;
        assert_eq!(type_name_of_val(&my_fn1), type_name_of_val(&my_fn2));
    }

}
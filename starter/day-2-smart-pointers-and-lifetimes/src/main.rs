// Box<T>  is smart pointer  that provides heap allocation for values.
// Rc<T>, Arc<T> , RefCell<T>

// fn main() {
//     let b = Box::new(10); // now this 10 is stored in heap
//     println!("b = {}", b);
// }

// lifetimes
// 1 = we use lifetime to avoid dangling refernces
// 2 = lifetime auto reject code which might lead to memory safety issues
// 3 = whenever we use reference in code there might be a chance for dangling references so we need lifetime

// fn main(){
//     let x = 10;
//     let y = get_val();

//     println!("{}", x + y);

// }

// fn get_val()-> i32 {
//     let y = 5;
//     y
// }


// lifetime error 
// fn main() {
//     let z = sum();
//     println!("{}", z);
// }

// fn sum<'a>() -> &'a i32 {
//     let x = 10;
//     &x
// }


// error for shorter lifetime 
// fn main() {
//     let result;

//     {
//         let x = 10;
//         let y = 20;
//         result = sum(&x, &y);
//     }

//     println!("{}", result);
// }

// fn sum<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
//     if x > y { x } else { y }
// }



// borrowed value does not live long enough
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("{}", r);
}
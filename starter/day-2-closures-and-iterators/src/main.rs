
use std::collections::HashMap;

//closures
fn main(){
    let mut counter = 0;

    let mut increase_counter = ||{
        counter = counter+1;
        println!("{}", counter)
    };

    increase_counter();
    increase_counter();
    increase_counter();
}


// // iterator map 
 fn main(){
    let vec = vec![1,2,3,4,5,6];

    let double_vec: Vec<i32> = vec.iter().map(|x| x + 2).collect();
    println!("{:?}", double_vec);

 }


// hashmaps


fn main(){
   let mut students: HashMap<String, u32> = HashMap::new();
   students.insert(String::from("Sameer"), 99);
   students.insert(String::from("Manish"), 89);
   students.insert(String::from("Ankit"), 79);

   for(student , marks) in students.iter(){
      println!("student name : {:?}, marks: {}", student, marks);
   }

}
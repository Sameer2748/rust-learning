struct Counter {
    count: u32 
}
impl Counter {
    fn new()-> Self {
        Self { count : 0} 
    }
}
// now we will impl nect fn for this counter struct 
impl Iterator for Counter {
    // now we are defining the trait iterator and for this traoit it need a fn named next that is mandatory 
    // if u wanna use this trait and we define this Self::Item  as agenric so we can define iit as we need 
    // it cna be u32 , i32, float or any u need so we need to define these 
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count = self.count + 1;
            Some(self.count)
        }else {
            None
        }
    }
}

fn main(){
    let mut count = Counter::new();
    // println!("{:?}", count.next());
    // println!("{:?}", count.next());
    // println!("{:?}", count.next());
    // println!("{:?}", count.next());
   
   for item in count {
    println!("{}", item);
   }
}
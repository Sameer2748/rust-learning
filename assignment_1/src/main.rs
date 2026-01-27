
trait Course {
    fn get_overview(&self) -> String;
}
struct Workshop {
    title: String,
    instructor: String,
    duration: u32
}
struct Seminar {
    title: String,
    speaker: String,
    location: String
}


impl Course for Workshop {
    fn get_overview(&self)-> String{
        format!("Workshop {} by {} for {} hours ", self.title, self.instructor, self.duration )
    }
}
impl Course for Seminar {
    fn get_overview(&self)-> String{
        format!("Seminar {} by {} at {} ",self.title, self.speaker, self.location)
    }
}

//   now here after this generic T we added Course text  and that is trait we bounded this trait to this T type paramter  so it can use tghe get_Overview fn that is defined in that trait 
fn print_overview<T: Course>(item: T){
    println!("{}", item.get_overview());
}


fn main() {
    let workshop1 = Workshop {
        title: String::from("Rust Programming"),
        instructor: String::from("John Doe"),
        duration: 10
    };
    let seminar1 = Seminar {
        title: String::from("Rust Programming"),
        speaker: String::from("Jane Doe"),
        location: String::from("New York")
    };
    print_overview(workshop1);
    // println!("{}", workshop1.get_overview());
    // println!("{}", seminar1.get_overview());
}
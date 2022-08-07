pub trait A {
    fn print(&self);
    
}
pub trait B {
    fn print(&self);
    
}

struct Mytype;

impl Mytype {
    fn print(&self) {
        println!("im in my type.");
    }
}
impl A for Mytype {
    fn print(&self) {
        println!("im from A.");
    }
}
impl B for Mytype {
    fn print(&self) {
        println!("im from B.");
    }
}



pub fn trait_demo(){
    let mt = Mytype;
    mt.print();
    A::print(&mt);
    B::print(&mt);
    <Mytype as A>::print(&mt);
    <Mytype as B>::print(&mt);


}
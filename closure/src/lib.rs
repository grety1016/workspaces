#![allow(dead_code)]
#![allow(unused)]

struct Carcher<T>
where T:Fn(u32) -> u32
{
    caculation:T,
    value:Option<u32>,
}

impl<T> Carcher<T>
where T:Fn(u32) -> u32
{
    fn new(caculation:T) -> Carcher<T>{
        Carcher { caculation, value: None, }
    }

    fn value(&mut self,arg:u32)  -> u32{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.caculation)(arg);
                self.value = Some(v);
                v
            }
            
        }
    }
}



pub fn closure(){
    // let mut s = Carcher::new(|x| x + 1);
    // let v = s.value(2);
    // println!("the v value is :{}",v);
    // let v2 = s.value(5);
    // println!("the v value is :{}",v2);


    
//  let v1 = vec![1,3,3,12,5,33,5,11,34];
//  let v2 = v1.into_iter();
//  let v3: Vec<_> = v2.filter(|x| *x > 10).collect();
//  println!("{:?}",v3);



}
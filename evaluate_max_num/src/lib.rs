 

pub fn lagest_i32(list: &[i32]) -> i32{
    let mut lagest:i32 = list[0];
    for &item in list  {
        if item > lagest{
            lagest = item;
        }        
    }
    lagest
}


pub fn lagest_char(list: &[char]) -> char{
    let mut lagest:char = list[0];
    for &item in list  {
        if item > lagest{
            lagest = item;
        }        
    }
    lagest
}

pub fn lagest_t<T>(list: &[T]) -> T
    where T:Copy + PartialOrd
{
    let mut lagest = list[0];
    for &item in list  {
        if item > lagest{
            lagest = item;
        }        
    }
    lagest
}


pub fn do_thing(){
    //数字求最大值
    let num_list = vec![1,2,3,4,5,6,7,8,9];
    let result_num  = lagest_t(&num_list);
    print!("the maxized num is {}\n",result_num);
    //字符求最大值 
    let char_list= vec!['c','f','r','Z','T','4','z'];
    let result_char  = lagest_t(&char_list);
    print!("the maxized num is {}\n",result_char);


}
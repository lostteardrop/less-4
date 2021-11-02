fn get_num(num:[u32;3])->u32{
    let mut newnum:u32=0;
    for i in num.iter(){
        newnum=newnum + i;
    }  
    newnum
}

fn main() {
    let len:u32;
    let num=[1,2,3];
    len=get_num(num);
    println!("{}",len);
}

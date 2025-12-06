// fn  borrow(a:String)->String{
//  println!("this is borrow");
//  return a  
// }

// fn main(){
//     let l=String::from("shihab");
//     let x=borrow(l);
//     println!("this is {}",x)
// }

//---------------- 2 ------------------

// fn borrow(a:String)->String{
//     println!("borrow");
//     return a
// }

// fn main(){
//     let l=String::from("yoyo");
//     let x=borrow(l);
//     println!("this is {}",x)
// }
//---------------- 3 ------------------


// fn borrow(s:String)->u32{
//     let x:u32=s.parse().expect("this is end");
//     let y:u32=10000;
//     let z:u32=x-y;
//     // println!("number {}",z)
//     return z
// }

// fn main(){
//     let l=String::from("202020");
//     let x=borrow(l);
//     println!("Number {}",x)
// }

//---------------- 4 ------------------

// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=1000;
//     let z:u32=x-y;
//     println!("divide {}",z);
//     return z;

// }

// fn main(){
//     let x=String::from("4999");
//     let f=borrow(x);
//     println!("number {}",f)
// }
//---------------- 5 ------------------

// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("borrow part");
//     let y:u32=1000;
//     let z:u32=x-y;
//     // println!("println {}",z)
//     return z
// }

// fn main(){
//     let x=String::from("1000000");
//     let t=borrow(x);
//     println!("number {}",t)
// }
//---------------- 6 ------------------

// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is ok");
//     let y:u32=12020;
//     let z:u32=x-y;
//     // println!("print {}",z);
//     return z;

// }
// fn main(){
//     let l=String::from("232222");
//     let y=borrow(l);
//     println!("ooo {}",y)
// }

//---------------- 7 ------------------

// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("the end");
//     let y:u32=1000;
//     let z:u32=x-y;
//     // println!("number {}",z);
//     return z
// }

// fn main(){
//     let l=String::from("2020200");
//     let z=borrow(l);
//     println!("Ooo {}",z)
// }

//---------------- 8 ------------------

// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is number");
//     let z:u32=200;
//     let u:u32=x+z;
//     println!("this end {}",u);
//     return u
// }

// fn main(){
//     let D=String::from("200000");
//     let w=borrow(D);
//     println!("oooo {}",w)
// }

//------------------ 9 --------------------

// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("number");
//     let y:u32=1000;
//     let z:u32=x-y;
//     return z

// }

// fn main(){
//     let l=String::from("200000");
//     let v=borrow(l);
//     println!("Olo {}",v)
// }

//------------------ 10 --------------------

fn borrow(a:String)->u32{
    let x:u32=a.parse().expect("this is not a number 🦀 \n");
    let y:u32=1000;
    let z:u32=x-y;
    println!("number {}",z);
    return z
}

fn main(){
    let l=String::from("fads100000d");
    let j=borrow(l);
    println!("ooo {}",j)
}
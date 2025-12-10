// fn borrow(a:&String){
//     let x:u32=a.parse().expect("this is end");
//     let y:u32=2000;
//     let z:u32=x-y;
//     println!("borrow {}",z)
// }

// fn main(){
//     let l=String::from("shihab");
//     borrow(&l);
//     println!("value {}",l)
// }

//-------------------------- 9 --------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is end");
//     let y:u32=2000;
//     let z:u32=x-y;
//     println!("value {}",z);
//     return z
// }

// fn main(){
//     let l =String::from("20000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//-------------------------- 8 --------------------------

// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=323;
//     let z:u32=x-y;
//     println!("value {}",z);
//     return z
// }

// fn main(){
//     let l=String::from("200000");
//     let x=borrow(l);
//     println!("value {}",x)
// }
//-------------------------- 7 --------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is end");
//     let y:u32=200;
//     let z:u32=x-y;
//     return z
// }

// fn main(){
//     let l=String::from("20000");
//     let x=borrow(l);
//     println!("value {}",x)
// }
//-------------------------- 6 --------------------------

// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("the end");
//     let y:u32=2000;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("2000000");
//     let t=borrow(l);
//     println!("value {}",t)
// }
//-------------------------- 5 --------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=1000;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("1000000");
//     let y=borrow(l);
//     println!("value {
//  } ",y)
// }
//-------------------------- 4 --------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=3000;
//     let z:u32=x*y;
//     return z
// }
// fn main(){
//       let l=String::from("1000000");
//     let y=borrow(l);
//     println!("value {
//  } ",y)  
// }
//-------------------------- 3 --------------------------

// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("the end");
//     let y:u32=2000;
//     let d:u32=x-y;
//     return d
// }
// fn main(){
//       let l=String::from("3000000");
//     let y=borrow(l);
//     println!("value {
//  } ",y)  
// }
//-------------------------- 2 --------------------------

// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=2000;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//           let l=String::from("3000000");
//     let y=borrow(l);
//     println!("value {
//  } ",y)  
// }
//-------------------------- 1 --------------------------
fn borrow(a:&String){
    let x:u32=a.parse().expect("this is the end");
    let y:u32=2000;
    let z:u32=x-y;

}
fn main(){
          let l=String::from("3000000");
    borrow(&l);
    println!("value {
 } ",l)  
}

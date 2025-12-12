// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=20000;
//     let z:u32=x-y;
//     return z
// }

// fn main(){
//     let l=String::from("30000000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//------------------------------ 9 ------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=1000;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("4000000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//------------------------------ 8 ------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=1000;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("10000000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//------------------------------ 7 ------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=2000;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("100000000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//------------------------------ 6 ------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=1500;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//     let l=String::from("3000000");
//     let y=borrow(l);
//     println!("value {}",y)
// }
//------------------------------ 5 ------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=20000;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//       let l=String::from("1000000");
//     let y=borrow(l);
//     println!("value {}",y)  
// }
//------------------------------ 4 ------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=1000;
//     let z:u32=x+y;
//     return z
// }
// fn main(){
//           let l=String::from("21000000");
//     let y=borrow(l);
//     println!("value {}",y)  
// }
//------------------------------ 3 ------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let  y:u32=1999;
//     let z:u32=x-y;
//     return z
// }
// fn main(){
//     let l=String::from("21000000");
//     let y=borrow(l);
//     println!("value {}",y) 
// }
//------------------------------ 2 ------------------------
// fn borrow(a:String)->u32{
//     let x:u32=a.parse().expect("this is the end");
//     let y:u32=999;
//     let z:u32=x*y;
//     return z
// }
// fn main(){
//         let l=String::from("300000");
//     let y=borrow(l);
//     println!("value {}",y) 
// }
//------------------------------ 1 ------------------------
fn borrow(a:&String){
    let x:u32=a.parse().expect("this is the end");
    let y:u32=999;
    let z:u32=x*y;
    
}
fn main(){
        let l=String::from("300000");
    borrow(&l);
    println!("value {}",l) 
}
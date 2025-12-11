// fn find_index(a:&[u32],tergat:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==tergat{
//             return Ok(index as u32)
//         }
//     }
//     Err("this is the end".to_string())
// }

// fn main(){
//     let arr=[2,34,5,67,8,90];
//     let tergat=67;
//     match find_index(&arr,tergat){
//         Ok(v)=>println!("value {}",v),
//         Err(e)=>println!("err {}",e)
//     }
// }
// ------------------------------- 14 ------------------------------

// fn find_index(a: &[u32], target: u32) -> Result<u32, String> {
//     for (index, &value) in a.iter().enumerate() {
//         if value == target {
//             return Ok(index as u32);
//         }
//     }
//     return Err("this is the end".to_string());  // Convert &str to String
// }

// fn main(){
//     let arr=[3,0,9,87,6,54];
//     let target=6;
//     match find_index(&arr,target){
//         Ok(v)=>println!("value {}",v),
//         Err(v)=>println!("value {}",v),
//     }
// }
// ------------------------------- 13 ------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index ,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }Err("this is the end".to_string())
// }
// fn main(){
//     let arr=[0,98,76,54,44,4];
//     let target=54;
//     match find_index(&arr,target){
//         Ok(v)=>println!("value {}",v),
//         Err(v)=>println!("value {}",v),
//     }
// }
// ------------------------------- 12 ------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }Err("this is the end".to_string())
// }
// fn main(){
//     let arr=[0,9,8,7,65,44,32];
//     let target=4;
//         match find_index(&arr,target){
//         Ok(v)=>println!("value {}",v),
//         Err(v)=>println!("value {}",v),
//     }
// }
// ------------------------------- 11 ------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index ,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }Err("this is the end".to_string())
// }
// fn main(){
//     let l=[5,6,56,54,2,54];
//     let target=4;
//             match find_index(&l,target){
//         Ok(v)=>println!("value {}",v),
//         Err(v)=>println!("value {}",v),
//     }
// }
// ------------------------------- 10 ------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index ,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }Err("this is the end".to_string())
// }
// fn main(){
//     let l=[5,6,56,54,2,54];
//     let target=4;
//             match find_index(&l,target){
//         Ok(v)=>println!("value {}",v),
//         Err(v)=>println!("value {}",v),
//     }
// }
// ------------------------------- 9 ------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index ,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }Err("this is the end".to_string())
// }
// fn main(){
//     let l=[5,6,56,54,2,54];
//     let target=4;
//             match find_index(&l,target){
//         Ok(v)=>println!("value {}",v),
//         Err(v)=>println!("value {}",v),
//     }
// }
// ------------------------------- 8 ------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index ,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }Err("this is the end".to_string())
// }
// fn main(){
//     let l=[5,6,56,54,2,54];
//     let target=4;
//             match find_index(&l,target){
//         Ok(v)=>println!("value {}",v),
//         Err(v)=>println!("value {}",v),
//     }
// }
// ------------------------------- 7 ------------------------------
// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }    
//     }Err("End".to_string())
// }
// fn main(){
//     let arr=[8,7,6,5,43,3];
//     let target=4;
//     match find_index(&arr,target){
//         Ok(v)=>println!("this is end {}",v),
//         Err(v)=>println!("this is end {}",v),
//     }
// }
// ------------------------------- 6 ------------------------------


// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{ return Ok(index as u32)
//     }}
//     Err("End".to_string())
// }

// fn main(){
//     let arr=[9,8,7,6,5,43,4];
//     let target=5;
//     match find_index(&arr,target){
//             Ok(v)=>println!("this is end {}",v),
//         Err(v)=>println!("this is end {}",v),    
//     }
// }
// ------------------------------- 5 ------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }
//     Err("end".to_string())
// }
// fn main(){
//     let arr=[6,57,6,43,32,3];
//     let target=6;
//     match find_index(&arr,target){
//                    Ok(v)=>println!("this is end {}",v),
//         Err(v)=>println!("this is end {}",v),    
//     }
// }
// ------------------------------- 4 ------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }Err("the end".to_string())
// }

// fn main(){
//     let arr=[57,64,5,4,5,6,4,5];
//     let target=4;
//     match find_index(&arr,target){
//         Ok(v)=>println!("this is  {}",v),
//         Err(v)=>println!("this is end {}",v),
//     }
// }
// ------------------------------- 3 ------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         return Ok(index as u32 )
//     }Err("end".to_string())
// }
// fn main(){
//     let arr=[57,64,5,4,5,6,4,5];
//     let target=4;
//     match find_index(&arr,target){
//         Ok(v)=>println!("this is  {}",v),
//         Err(v)=>println!("this is end {}",v),
//     }
// }
// ------------------------------- 2 ------------------------------
// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }
//     Err("this is end".to_string())
// }
// fn main(){
//     let arr=[7,89,64,77,3,45,6];
//     let target=3;
//         match find_index(&arr,target){
//         Ok(v)=>println!("this is  {}",v),
//         Err(v)=>println!("this is end {}",v),
//     }
// }
// ------------------------------- 1 ------------------------------
// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }
//     Err("this is end".to_string())
// }
// fn main(){
//     let arr=[7,89,64,77,3,45,6];
//     let target=0;
//         match find_index(&arr,target){
//         Ok(v)=>println!("this is  {}",v),
//         Err(v)=>println!("this is end {}",v),
//     }
// }

// ------------------------------- Option ------------------------------
// ------------------------------- 15 ------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target { 
//             return Some(index)
//     }}
//     None
// }
// fn main(){
//     let arr=[0,7,8,5,7,5,3];
//     let target=5;
//     match find_index(&arr,target){
//         Some(value)=>println!("value {}",value),
//         None=>println!("end")
//     }
// }
// ------------------------------- 14 ------------------------------

// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[6,5,67,4,56,6,4];
//     let target=54;
//     match find_index(&arr,target){
//         Some(value)=>println!("value {}",value),
//         None=>println!("end")  
//     }
// }
// ------------------------------- 13 ------------------------------

// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[34,5,7,8,9,5,8,7,9];
//     let target=6;
//     match find_index(&arr,target){
//         Some(value)=>println!("value {}",value),
//         None=>println!("end")  
//     }
// }
// ------------------------------- 12 ------------------------------

// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[6,4,5,3,2,52,5,4];
//     let target=4;
//     match find_index(&arr,target){
//         Some(value)=>println!("value {}",value),
//         None=>println!("end")     
//     }
// }
// ------------------------------- 11 ------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }

// fn main(){
//     let arr=[6,7,84,63,4,5,2,3];
//     let target=0;
//     match find_index(&arr,target){
//         Some(value)=>println!("value {}",value),
//         None=>println!("end")        
//     }
// }
// ------------------------------- 10 ------------------------------

// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[6,5,7,84,6,5,3,5,3];
//     let target=7;
//     match find_index(&arr,target){
//                Some(value)=>println!("value {}",value),
//         None=>println!("end")    
//     }
// }
// ------------------------------- 9 ------------------------------

// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//        let arr=[6,5,7,84,6,5,3,5,3];
//     let target=7;
//     match find_index(&arr,target){
//                Some(value)=>println!("value {}",value),
//         None=>println!("end")    
//     } 
// }
// ------------------------------- 8 ------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[6,5,7,84,6,5,3,5,3];
//     let target=3;
//     match find_index(&arr,target){
//                Some(value)=>println!("value {}",value),
//         None=>println!("end")    
//     }  
// }
// ------------------------------- 7 ------------------------------

// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//      let arr=[6,5,7,84,6,5,3,5,3];
//     let target=3;
//     match find_index(&arr,target){
//                Some(value)=>println!("value {}",value),
//             
// }
// ------------------------------- 6 ------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[4,5,7,6,45,6,5,3,46];
//     let target=9;
//         match find_index(&arr,target){
//                Some(value)=>println!("value {}",value),
//         None=>println!("end")    
//     }
// }
// ------------------------------- 5 ------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[2,3,5,24,5,32,4,31,45];
//     let target=4;
//             match find_index(&arr,target){
//                Some(value)=>println!("value {}",value),
//         None=>println!("end")    
//     } 
// }
// ------------------------------- 4 ------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{

//         for(index,&value) in a.iter().enumerate(){
//             if value==target{
//                 return Some(index)
//             }
//         }None
// }
// fn main(){
//     let arr=[34,53,24,2,32,43,2,4];
//     let target=4;
//             match find_index(&arr,target){
//                Some(value)=>println!("value {}",value),
//         None=>println!("end")    
// }}
// ------------------------------- 3 ------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//        if value==target{ return Some(index)
//     }}
//     None
// }

// fn main(){
//     let arr=[5,8,67,35,6,45,4,5];
//     let target=6;
//                 match find_index(&arr,target){
//                Some(value)=>println!("value {}",value),
//         None=>println!("end")  
// }}
// ------------------------------- 2 ------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[3,46,23,45,62,34,52,3,4];
//     let target=64;
//     match find_index(&arr,target){
//                Some(value)=>println!("value {}",value),
//         None=>println!("end")  
//     }
// }
// ------------------------------- 1 ------------------------------
fn find_index(a:&[u32],target:u32)->Option<usize>{
    for(index,&value) in a.iter().enumerate(){
        if value==target{
            return Some(index)
        }
    }None
}

fn main(){
    let arr=[3,2,4,56,2,34,6,23,4,6];
    let target=6;
    match find_index(&arr,target){
               Some(value)=>println!("value {}",value),
        None=>println!("end")  
    }
}
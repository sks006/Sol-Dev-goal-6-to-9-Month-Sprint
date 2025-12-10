// fn divide(a:f32,b:f32)->Result<f32,String>{
//     if b==0.0{
//     Err("this is the end".to_string())
//     }else{
//         Ok(a/b)
//     }
// }
// fn main(){
//     let l=divide(12.33,4.4);
//     match l{
//         Ok(v)=>println!("this is {}",v),
//         Err(e)=>println!("this is {}",e),
//     }
// }
// --------------------------- 14 -------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{

//         for (index,&value) in a.iter().enumerate(){
//             if value==target{
//                 return Ok(index as u32)
//             }
//         }
//         Err("this is the end".to_string())

// }

// fn main(){
//     let arr=[3,3,445,54,36,87];
//     let target=36;
//     match find_index(&arr,target){
//         Ok(v)=>println!("value {}",v),
//         Err(e)=>println!("value {}",e),
//     }
// }
// --------------------------- 13 -------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//            return Ok(index as u32)
//         }
//     }
//     Err("this is the end".to_string())
// }

// fn main(){
//     let arr=[23,55,234,344,333];
//     let target=234;
//     match find_index(&arr,target){
//         Ok(v)=>println!("value {}",v),
//         Err(v)=>println!("error {}",v),
//     }
// }
// --------------------------- 12 -------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for (index ,&value) in a.iter().enumerate(){
//         if (value==target){
//             return Ok(index as u32)
//         }
//     }
//     Err("this is the end".to_string())
// }


// fn main(){
//     let arr=[34,33,21,35,74,87];
//     let target=74;
//     match find_index(&arr,target){
//         Ok(v)=>println!("value {}",v),
//         Err(v)=>println!("err {}",v),
//     }
// }
// --------------------------- 11 -------------------------------
// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }
//     Err("this is the end".to_string())
// }

// fn main(){
//     let arr=[34,33,21,35,74,87];
//     let target=74;
//     match find_index(&arr,target){
//         Ok(v)=>println!("value {}",v),
//         Err(v)=>println!("err {}",v),
//     }
// }
// --------------------------- 10 -------------------------------
// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }
//     Err("this is the end".to_string())
// }
// fn main(){
//     let arr=[44,87,65,34,23,7];
//     let target=43;
//     match find_index(&arr,target){
//         Ok(v)=>println!{"value {}",v},
//         Err(v)=>println!{"value {}",v},
//     }
// }
// --------------------------- 9 -------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }
//     Err("this is the end".to_string())
// }
// fn main(){
//     let arr=[23,4,234,23,4,5,23];
//     let target=5;
//     match find_index(&arr,target){
//         Ok(v)=>println!{"value {}",v},
//         Err(v)=>println!{"value {}",v}, 
//     }
// }
// --------------------------- 8 -------------------------------
// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }
//     Err("this is the end".to_string())
// }
// fn main(){
//     let arr=[2,34,253,4,34];
//     let target=45;
//     match find_index(&arr,target){
//         Ok(v)=>println!{"value {}",v},
//         Err(v)=>println!{"err {}",v},
//     }
// }
// --------------------------- 7 -------------------------------
// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }
//     Err("this is the end".to_string())
// }

// fn main(){
//     let arr=[3,25,32,4,344,35,6];
//     let target=4;
//     match find_index(&arr,target){
//         Ok(v)=>println!{"value {}",v},
//         Err(v)=>println!{"err {}",v},
//     }
// }
// --------------------------- 6 -------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }Err("this is the end".to_string())
// }

// fn main(){
//     let arr=[2,34,53,43,24,34];
//     let target=43;
//     match find_index(&arr,target){
//         Ok(v)=>println!{"value {}",v},
//         Err(v)=>println!{"err {}",v},  
//     }
// }
// --------------------------- 5 -------------------------------
// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32 )
//         }
//     }Err("this is the end".to_string())
// }
// fn main(){
//     let arr=[23,45,6,76,65,44];
//     let target=6;
//     match find_index(&arr,target){
//         Ok(v)=>println!{"value {}",v},
//         Err(v)=>println!{"err {}",v},   
//     }
// }
// --------------------------- 4 -------------------------------
// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         return Ok(index as u32)
//     }Err("this is the end".to_string())
// }
// fn main(){
//     let arr=[33,4,63,43,47,83,2];
//     let target=4;
//     match find_index(&arr,target){
//         Ok(v)=>println!{"value {}",v},
//         Err(v)=>println!{"err {}",v},  
//     }
// }
// --------------------------- 3 -------------------------------
// fn find_index(a:&[u32],t:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==t{
//         return Ok( index as u32)
//     }
// }Err("this is the end".to_string())
// }
// fn main(){
//     let arr=[4,23,43,45,13,34,34];
//     let target=43;
//     match find_index(&arr,target){
//         Ok(v)=>println!{"value {}",v},
//         Err(v)=>println!{"err {}",v},      
//     }
// }
// --------------------------- 2 -------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if(value==target){
//             return Ok(index as u32)
//         }
//     }Err("this is the end".to_string())
// }
// fn main(){
//     let arr=[32,64,54,34,32,34];
//     let target=43;
//     match find_index(&arr,target){
//         Ok(v)=>println!{"value {}",v},
//         Err(v)=>println!{"err {}",v},      
//     }
// }
// --------------------------- 1 -------------------------------

// fn find_index(a:&[u32],target:u32)->Result<u32,String>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Ok(index as u32)
//         }
//     }Err("this is the end".to_string())
// }
// fn main(){
//     let arr=[34,54,35,64,53,67];
//     let target=64;
//         match find_index(&arr,target){
//         Ok(v)=>println!{"value {}",v},
//         Err(v)=>println!{"err {}",v},      
//     }
// }
// --------------------------- option -------------------------------
// --------------------------- 15 -------------------------------


// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for (index,&value) in a.iter().enumerate(){
//         if value==target{
//            return Some(index);
//         }
//     }
//     None
// }

// fn main(){
//     let arr=[23,24,23,4,21,3];
//     let target=4;
//             match find_index(&arr,target){
//         Some(v)=>println!{"value {}",v},
//         None=>println!{"err "},
// }
// }
// --------------------------- 14 -------------------------------

// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for (index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//      let arr=[23,24,23,4,21,3];
//     let target=4;
//             match find_index(&arr,target){
//         Some(v)=>println!{"value {}",v},
//         None=>println!{"err "},
// }   
// }
// --------------------------- 13 -------------------------------

// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//          let arr=[23,24,23,4,21,3];
//     let target=4;
//             match find_index(&arr,target){
//         Some(v)=>println!{"value {}",v},
//         None=>println!{"err "},
// } 
// }
// --------------------------- 12 -------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for (index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }
//     None
// }
// fn main(){
//          let arr=[23,24,23,4,21,3];
//     let target=21;
//             match find_index(&arr,target){
//         Some(v)=>println!{"value {}",v},
//         None=>println!{"err "},
// } 
// }
// --------------------------- 11 -------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }
//     None
//  }

//  fn main(){
//     let arr=[32,18,34,89,34,23];
//     let target=34;
//     match find_index(&arr,target){
//         Some(v)=>println!{"value {}",v},
//         None=>println!{"err "},
//     }
//  }
// --------------------------- 10 -------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[43,54,32,34,5,23,34];
//     let target=5;
//     match find_index(&arr,target){
//         Some(v)=>println!{"value {}",v},
//         None=>println!{"err "},
//     }
// }
// --------------------------- 9 -------------------------------

// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[32,45,32,43,23,44];
//     let target=23;
//         match find_index(&arr,target){
//         Some(v)=>println!{"value {}",v},
//         None=>println!{"err "},
//     }
// }
// --------------------------- 8 -------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[23,42,4,5,2,3,43,4];
//     let target=2;
//     match find_index(&arr,target){
//                Some(v)=>println!{"value {}",v},
//         None=>println!{"err "}, 
//     }
// }
// --------------------------- 7 -------------------------------

// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index ,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[3,4,56,3,4,5,6,44];
//     let target=5;
//     match find_index(&arr,target){
//         Some(v)=>println!{"value {}",v},
//         None=>println!{"err "},    
//     }
// }
// --------------------------- 6 -------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for (index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//         let arr=[3,4,56,3,4,5,6,44];
//     let target=5;
//     match find_index(&arr,target){
//         Some(v)=>println!{"value {}",v},
//         None=>println!{"err "},      
//     }
// }
// --------------------------- 5 -------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for (index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[34,5,2,4,345,2,34];
//     let target=3;
//     match find_index(&arr,target){
//         Some(v)=>println!{"value {}",v},
//         None=>println!{"err "},   
//     }
// }
// --------------------------- 4 -------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[34,8,2,3,45,3,24,3,45];
//     let target=24;
//     match find_index(&arr,target){
//         Some(v)=>println!{"value {}",v},
//         None=>println!{"err "}, 
//     }
// }
// --------------------------- 3 -------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for (index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }

// fn main(){
//        let arr=[34,8,2,3,45,3,24,3,45];
//     let target=24;
//     match find_index(&arr,target){
//         Some(v)=>println!{"value {}",v},
//         None=>println!{"err "}, 
//     }
// }
// --------------------------- 2 -------------------------------
// fn find_index(a:&[u32],target:u32)->Option<usize>{
//     for(index,&value) in a.iter().enumerate(){
//         if value==target{
//             return Some(index)
//         }
//     }None
// }
// fn main(){
//     let arr=[34,8,2,3,4,5,3,24,3,45];
//     let target=24;
//     match find_index(&arr,target){
//         Some(v)=>println!{"value {}",v},
//         None=>println!{"err "}, 
//     }
// }
// --------------------------- 1 -------------------------------
fn find_index(a:&[u32],target:u32)->Option<usize>{
    for(index,&value) in a.iter().enumerate(){
        if value==target{
            return Some(index)
        }
    }None
}
fn main(){
    let arr=[34,52,34,324,32,453,24,53,24,5];
    let target=24;
    match find_index(&arr,target){
        Some(v)=>println!("this is value :{}",v),
        None=>println!("this is error "),
    }
}
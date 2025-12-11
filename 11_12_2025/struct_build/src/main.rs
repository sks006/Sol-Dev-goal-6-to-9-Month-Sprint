// struct Loan{
//     account:u32,
//     inter:u32
// }
// impl Loan{
//     fn new (account:u32,inter:u32)->Self {
//         Self{account,inter}
//     }
//     fn inc_amount(&mut self )->u32{
//         self.account*=self.inter;
//         return self.account

//     }
// }
// fn main(){
//     let mut l=Loan::new(3333333,2);

//     println!("this is {}",l.account);
//     let x=l.inc_amount();
//     println!("this is {}",x);
// }
//-----------------------------19--------------------------------

// struct Loan{
//     amount:f32,
//     id:u32,
//     profit:u32
// }
// impl Loan{
//     fn new(id:u32,amount:f32,profit:u32)->Self{
//         Self{
//             id,amount,profit
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.amount+=self.profit as f32;
//         return self.amount
//     }
// }
// fn main(){
//     let mut l=Loan::new(23,3000.0,500);
//     println!("value :{}",l.amount);
//     let y= l.inc_amount();
//     println!("increase amount {}",y);
// }
//----------------------------- 18 --------------------------------

// struct Loan{
//     id:u32,
//     invest:f32,
//     profit:u32
// }
// impl Loan{
//     fn new(id:u32,invest:f32,profit:u32)->Self{
//         Self{
//             id,invest,profit
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest
//     }
// }

// fn main(){
//     let mut l=Loan::new(11,5000.8,500);
//     println!("invest {}",l.invest);
//     let x=l.inc_amount();
//     println!("total profit {}",x)
// }
//----------------------------- 17 --------------------------------
// struct Loan{
//     id:u32,
//     invest:f32,
//     profit:u32
// }

// impl Loan{
//     fn new(id:u32,invest:f32,profit:u32)->Self{
//         Self{
//             id,invest,profit
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest
//     }
// }

// fn main(){
//     let mut l=Loan::new(23,444444.4,4000);
//     println!("invest {}",l.invest);
//     let x=l.inc_amount();
//     println!("increase amount {}",x)
// }
//----------------------------- 16 --------------------------------
// struct Loan{
//     id:u32,
//     invest:f32,
//     profit:u32
// }

// impl Loan{
//     fn new(id:u32,invest:f32,profit:u32)->Self{
//         Self{
//             id,invest,profit
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest
//     }
// }

// fn main(){

//     let mut l=Loan::new(22,90000.9,2200);
//         println!("invest {}",l.invest);
//     let x=l.inc_amount();
//     println!("increase amount {}",x)
// }

//----------------------------- 15 --------------------------------
// struct Loan{
//        id:u32,
//     invest:f32,
//     profit:u32
// }
// impl Loan{
//     fn new(id:u32,invest:f32,profit:u32)->Self{
//         Self{
//             id,invest,profit
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest
//     }
// }
// fn main(){
//     let mut l=Loan::new(22,50000.9,2200);
//     println!("invest {}",l.invest);
//     let x=l.inc_amount();
//     println!("increase amount {}",x)
// }
//----------------------------- 14 --------------------------------
// struct Loan{
//     id:u32,
//     invest:f32,
//     profit:u32
// }

// impl Loan{
//     fn new(id:u32,invest:f32,profit:u32)->Self{
//         Self{
//             id,invest,profit
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest

//     }
// }

// fn main(){
//     let mut l=Loan::new(11,80000.22,5000);
//     println!("invest {}",l.invest);
//     let x=l.inc_amount();
//     println!("increase amount {}",x)
// }
//----------------------------- 13 --------------------------------

// struct Loan {
// id: u32,
// profit: u32,
// invest: f32,
// }
// impl Loan {
//     fn new(id: u32, profit: u32, invest: f32) -> Self {
//         Self {
//             id,
//             profit,
//             invest,
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest
//     }
// }

// fn main(){
//     let mut l=Loan::new(1,2000,200000.8);
//     println!("invest {}",l.invest);
//     let c=l.inc_amount();
//     println!("invest {}",c);
// }
//----------------------------- 12 --------------------------------

// struct Loan {
//     id: u32,
//     profit: u32,
//     invest: f32,
// }

// impl Loan {
//     fn new(id: u32, profit: u32, invest: f32) -> Self {
//         Self {
//             id,
//             profit,
//             invest,
//         }
//     }
//     fn inc_amount(&mut self) -> f32 {
//         self.invest += self.profit as f32;
//         return self.invest;
//     }
// }

// fn main() {
//     let mut l = Loan::new(2,3000,500000.0);
//     println!("invest {}",l.invest);
//     let x=l.inc_amount();
//     println!("invest {}",l.invest);
// }
//----------------------------- 11 --------------------------------

// struct Loan {
//     id: u32,
//     profit: u32,
//     invest: f32,
// }
// impl Loan {
//     fn new(id: u32, profit: u32, invest: f32) -> Self{
//          Self {
//             id,
//             profit,
//             invest
//         }
//     }

//     fn inc_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest
//     }
// }

// fn main(){
//    let mut l = Loan::new(2, 40000, 2000.0);
//     let x=l.inc_amount();
//     println!("total profit {}",x)
// }
//----------------------------- 10 --------------------------------
// struct Loan {
//     id: u32,
//     profit: u32,
//     invest: f32,
// }
// impl Loan {
//     fn new(id: u32, profit: u32, invest: f32)->Self{
//         Self{
//             id,profit,invest
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest-=self.profit as f32;
//         return self.invest
//     }
// }
// fn main(){
//     let mut l=Loan::new(3,500,400000.0);
//     let c=l.inc_amount();
//     println!("total profit {}",c)
// }
//----------------------------- 9 --------------------------------

// struct Loan {
//     id: u32,
//     profit: u32,
//     invest: f32,
// }
// impl Loan {
//     fn new(id: u32, profit: u32, invest: f32)->Self{
//         Self{
//             id,profit,invest
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest-=self.profit as f32;
//         return self.invest
//     }
// }
// fn main(){
//     let mut l=Loan::new(2,5000,30000000.0);
//     let x=l.inc_amount();
//     println!("total profit {}",x)
// }
//----------------------------- 8 --------------------------------
// struct Loan {
//     id: u32,
//     profit: u32,
//     invest: f32,
// }

// impl Loan {
//     fn new(id: u32, profit: u32, invest: f32)->Self{
//         Self{
//             id,profit,invest
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest
//     }
//     fn dec_amount(&mut self)->f32{
//         self.invest-=self.profit as f32;
//         return self.invest
//     }
// }
// fn main(){
//     let mut l=Loan::new(2,5000,30000000.0);
//     let x=l.inc_amount();
//     let y=l.dec_amount();
//     let y=l.dec_amount();
//     println!("total profit {}",x);
//     println!("total profit {}",y)
// }
//----------------------------- 7 --------------------------------
// struct Loan {
//     id: u32,
//     profit: u32,
//     invest: f32,
// }
// impl Loan {
//     fn new(id: u32, profit: u32, invest: f32)->Self{
//         Self{
//             id,profit,invest
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest
//     }
//     fn dec_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest
//     }
// }

// fn main(){
//     let mut l=Loan::new(3, 4000,5000000.0);
//     let x=l.inc_amount();
//     let y=l.dec_amount();
//     println!("total profit {}",x);
//     println!("total profit {}",y)
// }
//----------------------------- 6 --------------------------------

// struct Loan {
//     id: u32,
//     profit: u32,
//     invest: f32,
// }

// impl Loan {
//     fn new(id: u32, profit: u32, invest: f32)->Self{
//         Self{
//             id,profit,invest
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest
//     }
//     fn dec_amount(&mut self)->f32{
//         self.invest-=self.profit as f32;
//         return self.invest
//     }
// }
// fn main(){
//     let mut l=Loan::new(4,300,5000000.0);
//     let x=l.inc_amount();
//     let y=l.dec_amount();
//     println!("total profit {}",x);
//     println!("total profit {}",y)
// }
//----------------------------- 5 --------------------------------

// struct Loan {
//     id: u32,
//     profit: u32,
//     invest: f32,
// }

// impl Loan {
//     fn new(id: u32, profit: u32, invest: f32)->Self{
//         Self{
//             id,profit,invest
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest
//     }
//     fn dec_amount(&mut self)->f32{
//         self.invest-=self.profit as f32;
//         return self.invest
//     }
// }

// fn main(){
//     let mut l=Loan::new(4,300,5000000.0);
//     let x=l.inc_amount();
//     let y=l.dec_amount();
//     println!("total profit {}",x);
//     println!("total profit {}",y)
// }
//----------------------------- 4 --------------------------------

// struct Loan {
//     id: u32,
//     profit: u32,
//     invest: f32,
// }

// impl Loan {
//     fn new(id: u32, profit: u32, invest: f32)->Self{
//         Self{
//             id,profit,invest
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest
//     }
//     fn dec_amount(&mut self)->f32{
//         self.invest-=self.profit as f32;
//         return self.invest
//     }
// }
// fn main(){
//     let mut l=Loan::new(8,500,500000.0);
//     let x=l.inc_amount();
//     let y=l.dec_amount();
//     println!("total profit {}",x);
//     println!("total profit {}",y)
// }

//----------------------------- 3 --------------------------------

// struct Loan {
//     id: u32,
//     profit: u32,
//     invest: f32,
// }
// impl Loan {
//     fn new(id: u32, profit: u32, invest: f32)->Self{
//         Self{
//             id,profit,invest
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest
//     }
//     fn dec_amount(&mut self)->f32{
//         self.invest-=self.profit as f32;
//         return self.invest
//     }

// }
// fn main(){
//     let mut l=Loan::new(2,345,345322.3);
//     let mut l=Loan::new(8,500,500000.0);
//     let x=l.inc_amount();
//     let y=l.dec_amount();
//     let y=l.dec_amount();
//     println!("total profit {}",x);
//     println!("total profit {}",y)
// }
//----------------------------- 2 --------------------------------

// struct Loan {
//     id: u32,
//     profit: u32,
//     invest: f32,
// }
// impl Loan {
//     fn new(id: u32, profit: u32, invest: f32)->Self{
//         Self{
//             id,profit,invest
//         }
//     }
//     fn inc_amount(&mut self)->f32{
//         self.invest+=self.profit as f32;
//         return self.invest
//     }
//     fn dec_amount(&mut self)->f32{
//         self.invest-=self.profit as f32;
//         return self.invest
//     }

// }
// fn main(){
//     let mut l=Loan::new(2,345,345322.3);
//     let mut j=Loan::new(8,500,500000.0);
//     let x=l.inc_amount();
//     let y=j.dec_amount();
//     let y=l.dec_amount();
//     println!("total profit {}",x);
//     println!("total profit {}",y)
// }
//----------------------------- 1 --------------------------------

struct Loan {
    id: u32,
    profit: u32,
    invest: f32,
}

impl Loan {
    fn new(id: u32, profit: u32, invest: f32) -> Self {
        Self {
            id,profit,invest
        }
    }
    fn inc_amount(&mut self)->f32{
        self.invest+=self.profit as f32;
        return self.invest
    }
    fn dec_amount(&mut self)->f32{
        self.invest=self.profit as f32;
        return self.invest
    }
}

fn main(){
    let mut l=Loan::new(2,345,345322.3);
    let mut j=Loan::new(8,500,500000.0);
    let x=l.inc_amount();
    let y=j.dec_amount();
    let y=l.dec_amount();
    println!("total profit {}",x);
    println!("total profit {}",y)
}

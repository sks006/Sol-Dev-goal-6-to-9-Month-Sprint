//----------------------------- 1 -------------------------------------------

// enum Status {
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self { acc: a, status: s }
//     }
// }
// fn main(){
//     let  l=Loan::new(2222,Status::pass);
//     match l.status{
//         Status::pass=>println!("Pass {}",l.acc),
//         Status::reje=>println!("Reje {}",l.acc),
//     }
// }

//----------------------------- 2 -------------------------------------------
// enum Status{
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(3333,Status::reje);
//     match l.status{
//         Status::pass=>println!("Pass {}",l.acc),
//         Status::reje=>println!("Reje {}",l.acc),}
// }
//----------------------------- 3 -------------------------------------------
// enum Status{
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(3333,Status::reje);
//     match l.status{
//         Status::pass=>println!("Pass {}",l.acc),
//         Status::reje=>println!("Reje {}",l.acc),
//     }
// }
//----------------------------- 4 -------------------------------------------
// enum Status{
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(2200,Status::pass);
//     match l.status{
//         Status::pass=>println!("Pass {}",l.acc),
//         Status::reje=>println!("Reje {}",l.acc),
//     }
// }
//----------------------------- 5 -------------------------------------------
// enum Status{
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }

// fn main(){
//     let l=Loan::new(555,Status::reje);
//     match l.status{
//         Status::pass=>println!("Pass {}",l.acc),
//         Status::reje=>println!("Reje {}",l.acc),
//     }
// }
//----------------------------- 6 -------------------------------------------
// enum Status{
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(3232,Status::pass);
//     match l.status{
//         Status::pass=>println!("Pass {}",l.acc),
//         Status::reje=>println!("Reje {}",l.acc),
//     }
// }
//----------------------------- 7 -------------------------------------------
// enum Status{
//     pass,
//     reje
// }
// struct Loan{
//     acc:u32,
//     status:Status
// }
// impl Loan{
//     fn new (a:u32,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(321,Status::pass);
//     match l.status{
//         Status::pass=>println!("Pass {}",l.acc),
//         Status::reje=>println!("Reje {}",l.acc),
//     }
// }
//----------------------------- 8 -------------------------------------------
// enum Status{
//     Pass,
//     Reje
// }
// struct Loan{
//     amount:u32,
//     status:Status,
//     share:u32
// }
// impl Loan{
//     fn new(a:u32,s:Status,sh:u32)->Self{
//         Self{
//             amount:a,
//             status:s,
//             share:sh
//         }
//     }
//     fn inc_amo(&mut self,inc:u32){
//         self.amount*=inc;
//     }
// }
// fn main(){
//     let mut l=Loan::new(1000,Status::Pass,10);
//     match l.status{
//         Status::Pass=>println!("Pass {}",l.amount),
//         Status::Reje=>println!("Reje {}",l.amount), }
//     let mut h = Loan::new(200, Status::Pass, 10);
    
//     // 正确调用 inc_amo 方法
//     h.inc_amo(10);
//     l.inc_amo(5);
    
//     println!("inc amo {}", h.amount);
//     println!("inc amo {}", l.amount);
// }
//----------------------------- 9 -------------------------------------------
// enum Status{
//     Pass,
//     Reje
// }
// struct Loan{
//     amount:u32,
//     status:Status
// }
// impl Loan{
//     fn new(a:u32,s:Status)->Self{
//         Self{
//             amount:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(2000,Status::Reje);
//     match l.status{
//         Status::Pass=>println!("Pass {}",l.amount),
//         Status::Reje=>println!("Reje {}",l.amount),
//     }
// }
//----------------------------- 10 -------------------------------------------
// enum Status{
//     Pass,
//     Reje
// }
// struct Loan{
//     acc:u64,
//     status:Status,
// }
// impl Loan{
//     fn new(a:u64,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(300,Status::Pass);
//     match l.status{
//         Status::Pass=>println!("Pass {}",l.acc),
//         Status::Reje=>println!("Reje {}",l.acc),
//     }
// }
//----------------------------- 11 -------------------------------------------
// enum Status{
//     Pass,
//     Reje
// }
// struct Loan{
//     acc:u64,
//     status:Status
// }
// impl Loan{
//     fn new(a:u64,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(555,Status::Reje);
//     match l.status{
//         Status::Pass=>println!("Pass {}",l.acc),
//         Status::Reje=>println!("Reje {}",l.acc),
//     }
// }
//----------------------------- 12 -------------------------------------------
// enum Status{
//     pass,
//     reje
// }
// struct Loan{
//     acc:u64,
//     status:Status
// }
// impl Loan{
//     fn new(a:u64,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(2332,Status::pass);
//     match l.status{
//         Status::pass=>println!("pass {}",l.acc),
//         Status::reje=>println!("reje {}",l.acc),
//     }
// }

//----------------------------- 13 -------------------------------------------
// enum Status{
//     pass,
//     reje
// }
// struct Loan{
//     acc:u64,
//     status:Status
// }
// impl Loan{
//     fn new(a:u64,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(777,Status::reje);
//     match l.status{
//         Status::pass=>println!("pass {}",l.acc),
//         Status::reje=>println!("reje {}",l.acc),
//     }
// }
//----------------------------- 14 -------------------------------------------
// enum Status{
//     pass,
//     reje
// }
// struct Loan{
//     acc:u64,
//     status:Status
// }
// impl Loan{
//     fn new(a:u64,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }
// fn main(){
//     let l=Loan::new(9000,Status::pass);
//     match l.status{
//         Status::pass=>println!("pass {}",l.acc),
//         Status::reje=>println!("reje {}",l.acc),}
// }
//----------------------------- 15 -------------------------------------------
// enum Status{
//     pass,
//     reje
// }
// struct Loan{
//     acc:u64,
//     status:Status
// }
// impl Loan{
//     fn new(a:u64,s:Status)->Self{
//         Self{
//             acc:a,
//             status:s
//         }
//     }
// }

// fn main(){
//     let l=Loan::new(12,Status::reje);
//     match l.status{
//         Status::pass=>println!("pass {}",l.acc),
//         Status::reje=>println!("reje {}",l.acc),
//     }
// }
//----------------------------- 16 -------------------------------------------
// enum Status{
//     Pass,
//     Reje
// }
// struct Loan{
//     amount:u32,
//     status:Status,
//     acc:u32,
// }
// impl Loan{
//     fn new(a:u32,s:Status,ac:u32)->Self{
//         Self{
//             amount:a,
//             status:s,
//             acc:ac
//         }
//     }
//     fn inc_amo(&mut self,inc:u32){
//         self.amount*=inc;
//     }
// }
// fn main(){
//     let mut l=Loan::new(4500,Status::Pass,23);
//     match l.status{
//         Status::Pass=>println!("pass {} {}",l.amount,l.acc),
//         Status::Reje=>println!("reje {} {}",l.amount,l.acc),
//     };
//     l.inc_amo(4);
//     println!("inc amo {}",l.amount);

// }

//----------------------------- 17 -------------------------------------------
// enum Status{
//     Pass,
//     Reje
// }
// struct Loan{
//     amount:u32,
//     status:Status,
//     acc:u32,
// }
// impl Loan{
//     fn new(a:u32,s:Status,ac:u32)->Self{
//         Self{
//             amount:a,
//             status:s,
//             acc:ac
//         }
//     }
//     fn inc_amo(&mut self,inc:u32){
//         self.amount*=inc;
//     }
// }

// fn main(){
//     let mut l=Loan::new(3333,Status::Reje,45);
//     match l.status{
//         Status::Pass=>println!("pass {} {}",l.amount,l.acc),
//         Status::Reje=>println!("reje {} {}",l.amount,l.acc),
//     }
//     l.inc_amo(3);
//     println!("inc amo {}",l.amount);
// }

//----------------------------- 18 ----------------------------------------
// enum Status{
//     Pass,
//     Reje
// }
// struct Loan{
//     amount:u32,
//     status:Status,
//     acc:u32,
// }
// impl Loan{
//     fn new(a:u32,s:Status,ac:u32)->Self{
//         Self{
//             amount:a,
//             status:s,
//             acc:ac
//         }
//     }
//     fn inc_amo(&mut self,inc:u32){
//         self.amount*=inc;
//     }
// }
// fn main(){
//     let mut l=Loan::new(200,Status::Pass,67);
//     match l.status{
//         Status::Pass=>println!("pass {} {}",l.amount,l.acc),
//         Status::Reje=>println!("reje {} {}",l.amount,l.acc),
//     };
//     l.inc_amo(6);
//     println!("inc amo {}",l.amount);

// }
//----------------------------- 19 ----------------------------------------
// enum Status{
//     Pass,
//     Reje
// }
// struct Loan{
//     amount:u32,
//     status:Status,
//     acc:u32,
// }
// impl Loan{
//     fn new (a:u32,s:Status,ac:u32)->Self{
//         Self{
//             amount:a,
//             status:s,
//             acc:ac
//         }
//     }
//     fn inc_amo(&mut self,inc:u32){
//         self.amount*=inc;}
// }

// fn main(){
//     let mut l=Loan::new(1500,Status::Reje,20);
//     match l.status{
//         Status::Pass=>println!("pass {} {}",l.amount,l.acc),
//         Status::Reje=>println!("reje {} {}",l.amount,l.acc),
//     }
//     l.inc_amo(2);
//     println!("inc amo {}",l.amount);
// }
//----------------------------- 20 -------------------------------------------
enum Status{
    pass,
    reje
}
struct Loan{
    acc:u32,
    status:Status,
    amount:u32,
}

impl Loan{
    fn new(a:u32,s:Status,ac:u32)->Self{
        Self{
            acc:a,
            status:s,
            amount:ac
        }
    }
    fn inc_amo(&mut self,inc:u32){
        self.amount*=inc;
    }
}

fn main(){
    let mut l=Loan::new(300,Status::reje,40);
    match l.status{
        Status::pass=>println!("pass {} {}",l.acc,l.amount),
        Status::reje=>println!("reje {} {}",l.acc,l.amount),
    }
    l.inc_amo(3);
    println!("inc amo {}",l.amount);
}
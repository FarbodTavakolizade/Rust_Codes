#[derive(Debug)]

struct BankAccount{
   owner:String,
   account_number:u32,
   balance:f64,
}

struct BankAccountBuilder {
  owner:String,
   account_number:u32,
   balance:f64,
}

impl BankAccountBuilder{
   fn new(owner:&str , account_number:u32) ->Self{
      Self{
      owner:owner.to_string(),
      account_number,
      balance:0.0,
      }
   }
    fn balance(mut self, amount:f64) ->Self{
         self.balance =amount;
         self
      
    }

    fn build(self) ->BankAccount{
      BankAccount{
         owner:self.owner,
         account_number:self.account_number,
         balance:self.balance,
      }
    }
}



fn main() {
    let account =BankAccountBuilder::new("alireza",21454521)
    .balance(2000.0)
    .build();

   println!("{:#?}",account);
}
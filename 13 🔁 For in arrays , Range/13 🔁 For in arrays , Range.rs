fn main() {
   let month_days=1..=31;
   println!("{month_days:#?}");
   for numbers in month_days{
      println!("{numbers}");
   }


   let alphabetic_char='a'..='z';

   for letters in alphabetic_char{
      println!("{letters}");
   }


   let names=["reza", "ali", "farbod"];
   for a in names{
      println!("{a}");
   }

   println!("{names:?}");
} 

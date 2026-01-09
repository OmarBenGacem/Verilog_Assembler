use std::fmt;
use std::println;
use crate::tokens::literal::Literal;
// #[allow(dead_code, unused_variables, unused_imports)]

pub struct Number {
    pub label : String,
    pub width : u32,
    pub value : u32,
}


pub fn create_number(num: Literal) -> Number {
        let width = match num {
            Literal::Binary { width, .. } => width,
            Literal::Hex { width, .. } => width,
            Literal::Decimal { width, .. } => width,
        };

      println!("{:?}", num);
      let max_unsigned_range : u64 = 1 << width;

      match num {

        Literal::Binary {..} => {
            
        },
        Literal::Hex {..} => {

        },
        Literal::Decimal {..} => {

        }

      }

    Number {label: "test".to_string(), width: 2, value: 3}
}


impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {} ({} digits)",self.label , self.value, self.width)
    }
}

// impl Number {

//     fn value(&self) -> u32 {
//         self.value
//     }
//     fn binary_representation(&self) -> u64 {
//         0
//     }

//     fn hex_respresentation(&self) -> u64 {
//         0
//     }

//     fn unsigned_decimal(&self) -> u64 {
//         0
//     }

//     fn signed_decimal(&self) -> i64 {
//         0
//     }
// }

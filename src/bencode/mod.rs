pub use order_map::OrderMap;
use std::cmp::Ordering;

#[derive(Debug)]
pub enum Bobject<'a> {
    Bstring(String),
    Bint(i32),
    Blist(Vec<Bobject<'a>>),
    Bdict(OrderMap<'a, Bobject<'a>>),
}

impl<'a> Bobject<'a> {
    pub fn bencode(&self) {
        match self {
            Bobject::Bstring(bs) => println!("{}", bs),
            Bobject::Bint(bi) => println!("{}", bi),
            Bobject::Blist(bl) => println!("{:#?}", bl),
            Bobject::Bdict(bd) => println!("{:#?}", bd),
        }
    }
}

mod order_map;

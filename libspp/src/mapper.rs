extern crate bimap;
use bimap::*;

pub struct SppMapper<'a> {
    mp: BiMap<u16, &'a str>,
    cnt: u16
}

impl SppMapper {
    pub fn new<'a>()-> SppMapper<'a> {
        SppMapper {
            mp: bimap::BiMap::new(),
            cnt: 0
        }
    }
}

impl <'a> SppMapper<'a> {
    pub fn string_to_integer(&self, id: &'a str) -> Option<u16> {
        self.mp.get_by_right(&id).map(|u| *u)
    }

    pub fn integer_to_string(&self, id: u16) -> Option<&'a str> {
        self.mp.get_by_left(&id).map(|u| *u)
    }

    pub fn define_string_id(&mut self, id: &'a str) {
        self.cnt = self.cnt + 1;
        self.mp.insert(self.cnt, id);
    }
}




extern crate bimap;

static MAGIC: [u8; 3] = [0x53, 0x50, 0x50];

static PK_STATE_STRING: u8 = 0x01;
static PK_STATE_INTEGER: u8 = 0x02;

use bimap::*;

pub fn new_mapper<'a>()-> SppMapper<'a> {
    SppMapper {
        mp: bimap::BiMap::new(),
        cnt: 0
    }
}

pub struct SppMapper<'a> {
    mp: BiMap<usize, &'a str>,
    cnt: usize
}

impl <'a> SppMapper<'a> {
    pub fn string_to_integer(&self, id: &'a str) -> Option<usize> {
        self.mp.get_by_right(&id).map(|u| *u)
    }

    pub fn integer_to_string(&self, id: usize) -> Option<&'a str> {
        self.mp.get_by_left(&id).map(|u| *u)
    }

    pub fn define_string_id(&mut self, id: &'a str) {
        self.cnt = self.cnt + 1;
        self.mp.insert(self.cnt, id);
    }
}


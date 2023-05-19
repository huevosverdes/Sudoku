use string_builder::Builder;

#[derive(Copy, Clone)]
pub struct Cell {
    c : char,
    opt : CellOptions,
}

impl Cell {
    pub fn new() -> Cell {
        return Cell { c : ' ', opt : CellOptions::new() };
    }

    pub fn clear(&mut self) {
        self.c = ' ';
        self.opt.clear();
    }

    pub fn character(&self) -> char {
        return self.c;
    }

    pub fn options(&self) -> &CellOptions {
        return &self.opt;
    }

    pub fn set(&mut self, c : char) {
        self.c = c;
        self.opt.set_one(c);
    }

    pub fn is_set(&self) -> bool {
        return '1' <= self.c && self.c <= '9';
    }

    pub fn apply_option(&mut self) {
        assert!(self.has_one_option());
        self.c = self.opt.get_last_option();
    }

    pub fn remove_option(&mut self, c : char) {
        self.opt.unset(c);
    }

    pub fn has_one_option(&self) -> bool {
        return self.opt.has_one_option();
    }

    #[allow(dead_code)]
    pub fn display(&self) -> String {
        let result : String;
        if '1' <= self.c && self.c <= '9' {
            result = format!("{}", self.c);
        } else {
            result = self.opt.display();
        }
        return result;
    }
}

#[derive(Copy, Clone)]
pub struct CellOptions {
    value : u16,
}

impl CellOptions {
    const ALL : u16 = 0x1FF;

    pub fn new() -> CellOptions {
        return CellOptions { value : CellOptions::ALL };
    }

    pub fn clear(&mut self) {
        self.value = CellOptions::ALL;
    }

    pub fn set_one(&mut self, c : char) {
        self.value = 0x1 << (c as u32) - ('1' as u32);
    }

    pub fn has_one_option(&self) -> bool {
        // Test if only one bit has been set: Is power of 2?
        let v = self.value;
        if v > 0 {
            return v == 1 || ((v & (v - 1)) == 0);
        }
        return false;
    }

    pub fn get_last_option(&mut self) -> char {
        assert!(self.has_one_option());
        let result : char;
        match self.value {
        0x001 => result = '1',
        0x002 => result = '2',
        0x004 => result = '3',
        0x008 => result = '4',
        0x010 => result = '5',
        0x020 => result = '6',
        0x040 => result = '7',
        0x080 => result = '8',
        0x100 => result = '9',
        _     => result = ' ',
        }
        return result;
    }

    pub fn unset(&mut self, c : char) {
        self.value = self.value & !(0x1 << (c as u32) - ('1' as u32));
    }

    #[allow(dead_code)]
    pub fn union(&self, other : &CellOptions) -> CellOptions {
        let mut result = CellOptions::new();
        result.value = self.value | other.value;
        return result;
    }

    #[allow(dead_code)]
    pub fn intersection(&self, other : &CellOptions) -> CellOptions  {
        let mut result = CellOptions::new();
        result.value = self.value & other.value;
        return result;
    }

    pub fn difference(&self, other : &CellOptions) -> CellOptions  {
        let mut result = CellOptions::new();
        result.value = self.value & !other.value;
        return result;
    }

    pub fn display(&self) -> String {
        let mut builder = Builder::default();
        builder.append("{");
        let mut tmp = self.value;
        let mut offset = 0;
        let mut first = true;
        while tmp != 0 {
            if tmp & 0x01 != 0 {
                if !first {
                    builder.append(", ");
                } else {
                    first = false;
                }
                let s : String = char::from_u32(offset + ('1' as u32)).unwrap().into();
                builder.append(s);
            }
            tmp = tmp >> 1;
            offset += 1;
        }
        builder.append("}");
        return builder.string().unwrap();
    }
}

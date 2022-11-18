struct Positions{
    
    pub LASTCHAR: i32,
    pub MAX_KEY_POS: i32,
    pub MAX_SIZE: i32,
    _useall: bool,
    _size: u32,
    _positions: [i32; 256]
}


impl Default for Positions {

    fn default() -> Positions {
        Positions {
            LASTCHAR: -1,
            MAX_KEY_POS: -1,
            MAX_SIZE: MAX_KEY_POS + 1,
            _useall: false,
            _size: 0,
            _positions: [None; 256],
        }
    }

}


impl Positions {

    //parameters in new function
    // pub fn new() -> Positions {

    // }

    #[inline]
    pub const fn is_useall() -> bool {
        return _useall;
    }

    // pub const fn operator[] (index: u32) -> i32 {

    // }

    #[inline]
    pub const fn get_size() -> u32 {
        return _size;
    }

    #[inline]
    pub fn set_useall(useall: bool) {
        _useall = useall;
        if useall {
            _size = MAX_KEY_POS;
            let mut ptr: *mut i32 = _positions;
            let mut i: i32 = MAX_KEY_POS;
            while i >= 0  {
                *ptr += 1;
                i -= 1;
            }
        }
    }

    #[inline]
    pub fn pointer() -> *mut i32 {
        return _positions;
    }

    #[inline]
    pub fn set_size(size: u32) {
        _size = size;
    }


    #[inline]
    pub fn sort() -> bool {
        if _useall {
            return true;
        }

        let duplicate_free: bool = true;
        let mut base: *mut i32 = _positions;
        let mut len: u32 = _size;

        for i in 1..len {
            let mut j: u32;
            let mut tmp: i32;

            
        }

    }

    //Function overloading
    
    pub const fn iterator() -> PositionIterator {

    }
    
    // pub const fn iterator(maxlen: i32) -> PositionIterator {

    // }

    pub const fn reviterator() -> PositionReverseIterator {

    }

    // pub const fn reviterator(maxlen: i32) -> PositionReverseIterator {

    // }

    
    pub const fn contains(pos: i32) -> bool {
        let mut count: u32 = _size;
        let mut p: *const i32 = _positions + _size - 1;

        while count > 0 {
            if *p == pos {
                return true;
            }

            if *p > pos {
                break;
            }

            p -= 1;
            count -= 1;
        }
        return false;
    }


    pub fn add(pos: i32) {
        set_useall(false);

        let mut count: u32 = _size;

        if count == MAX_SIZE  {
            eprintln!("Positions::add internal error: overflow");
            std::process::exit(1);
        }

        let mut p: *mut i32 = _positions + _size - 1;

        while count > 0 {
            if *p == pos {
                eprintln!("Positions:add internal error: duplicate");
                std::process::exit(1);
            }

            if *p > pos {
                break;
            }

            p[1] = p[0];
            
            p -= 1;
            count -= 1;
        }

        p[1] = pos;
        _size += 1;
    }


    pub fn remove(pos: i32) {
        set_useall(false);

        let mut count: u32 = _size;

        if count > 0 {
            let mut p: *mut i32 = _positions + _size - 1;

            if *p == pos {
                _size -= 1;
                return;
            }

            if *p < pos {
                let mut prev: i32 = *p;

                while true {
                    p -= 1;
                    count -= 1;
                    if count == 0 {
                        break;
                    }
                    if *p == pos {
                        *p = prev;
                        _size -= 1;
                        return;
                    }
                    if *p > pos {
                        break;
                    }
                    let mut curr: i32 = *p;
                    *p = prev;
                    prev = curr;
                }

            }
        }
        eprintln!("Positions::remove internal error: not found");
        std::process::exit(1);
    }


    pub const fn print() {
        if _useall {
            print!("*");
        } else {
            let mut first: bool = true;
            let mut seen_LASTCHAR = false;
            let mut count: u32 = _size;
            let mut p: *const i32 = _positions + _size - 1;

            while count > 0 {
                count -= 1;
                if *p == LASTCHAR {
                    seen_LASTCHAR = true;
                } else {
                    if !first {
                        print!(",");
                    }
                    print("{}", *p + 1);
                    if count > 0 && p[-1] == *p + 1 {
                        print!("-");
                        loop {
                            p -= 1;
                            count -= 1;
                            if !(count > 0 && p[-1] == *p + 1) {
                                break;
                            }
                        }
                        print!("{}", *p + 1);
                    }
                    first = false;
                }

            }
            if seen_LASTCHAR {
                if !first {
                    print!(",");
                }
                print!("$");
            }
        }

    }

}


struct PositionIterator {

    pub EOS: i32,
    _set: &Positions,
    _index: u32
}

impl Default for PositionIterator {
    fn default() -> PositionIterator {
        
        PositionIterator {
            EOS: -2,
            //How to give default value to the data type - reference to other struct
            _index: -1
        }

    }
}

impl PositionIterator {

    // pub fn new() -> PositionIterator {

    // }

    pub fn next() -> i32 {

    }

    pub const fn remaining() -> u32 {

    }

    
}

//How to declare mutable fields

struct PositionReverseIterator {

    pub EOS: i32,
    _set: &Positions,
    _index: u32,
    _minindex: u32
}

impl Default for PositionReverseIterator {
    fn default() -> PositionReverseIterator {
        PositionReverseIterator {
            EOS: -2,
            //How to give default value to the data type - reference to other struct
            _index: -1,
            _minindex: -1
        }
    }
}


impl PositionReverseIterator {

    // pub fn new() -> PositionReverseIterator {
        
    // }

    pub fn next() -> i32 {

    }

    pub const fn remaining() -> u32 {

    }

}


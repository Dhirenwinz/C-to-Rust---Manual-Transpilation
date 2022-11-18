mod positions;
use std::process;


#[inline]
fn sort_char_set(base: *mut u32, len: i32) {

    for i in 1..len + 1 {
        
        let mut j: i32 = i;
        let tmp: u32 = base[j];

        while j > 0 && tmp < base[j - 1] {
            base[j] = base[j - 1];
            j -= 1;
        }

        base[j] = tmp;
    }

}

/* Declaring Keyword */
struct Keyword{
    _allchars: String,
    _allchars_length: i32,
    _rest: String,
    _lineno: u32
}


struct KeywordExt{
    keyword: Keyword,
    _selchars: *const u32,
    _selchars_length: i32,
    _duplicate_link: *mut KeywordExt,
}


impl KeywordExt{

    fn init_selchars_low(&positions: Positions, alpha_unify: *const u32, alpha_inc: *const i32) -> *mut u32 {

        let iter: PositionIterator = positions.iterator(_allchars_length);

        let key_set: *mut u32 = Box::into_raw(Box::new(iter.remaining())) as *mut u32;
        let mut i: i32;

        while i = iter.next() != PositionIterator::EOS {
            
            let mut c: u32;
            
            if i == Positions::LASTCHAR {
                c = _allchars[_allchars_length - 1] as u8;
            } else if i < _allchars_length {
                c = _allchars[i] as u8;
                if alpha_inc {
                    c += alpha_inc[i];
                }

            } else {

                process::abort();
                if alpha_unify {
                    c = alpha_unify[c];
                }

                *ptr = c;
                ptr += 1;
            
            }

        }

        _selchars = key_set;
        _selchars_length = ptr - key_set;

        return key_set;

    }


    fn init_selchars_tuple(&positions: Positions, alpha_unify: *const u32){
        init_selchars_low (positions, alpha_unify, std::ptr::null());
    }


    fn init_selchars_multiset(&positions: Positions, alpha_unify: *const u32, alpha_inc: *const u32){

        let mut selchars: *mut u32 = init_selchars_low (positions, alpha_unify, alpha_inc);
        
        sort_char_set(selchars, _selchars_length);
    }


    fn delete_selchars(){
        unsafe{
            std::mem::drop(Box::from_raw(_selchars));
        }
    }

}


struct Keyword_Factory {

}

static empty_string: [char; 1];
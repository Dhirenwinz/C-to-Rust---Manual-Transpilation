mod keyword;
mod options;

struct Hash_Table {
    _table: *mut *mut KeywordExt,
    _size: u32,
    _log_size: u32,
    _ignore_length: bool,
    _collisions: u32
}

const size_factor: i32 = 10;

impl Hash_Table {

    pub fn new(mut size: u32, mut ignore_length: bool) -> Hash_Table {
        
        size = size * factor;

        let mut shift: u32 = 0;

        if (size >> 16) > 0 {
            size = size >> 16;
            shift += 16;
        }

        if (size >> 8) > 0 {
            size = size >> 8;
            shift += 8;
        }

        if (size >> 4) > 0 {
            size = size >> 4;
            shift += 4;
        }

        if (size >> 2) > 0 {
            size = size >> 2;
            shift += 2;
        }

        if (size >> 1) > 0 {
            size = size >> 1;
            shift += 1;
        }

        _log_size = shift;
        _size = 1 << shift;

        _table = Box::into_raw(Box::new([0;_size])) as *mut *mut KeywordExt;
    }

    pub fn insert(mut item: *mut KeywordExt) -> *mut Keyword{
        
        //Know what is hashpjw function
        let mut hash_val: u32 = hashpjw(*item.selchars as *const u8, *item._selchars_length * std::mem::size_of::<u32>());
        let mut probe: u32 = hash_val & (_size - 1);
        let mut increment: u32 = (((hash_val >> _log_size) ^ (if _ignore_length > 0 {0} else {*item._allchars_length})) << 1) + 1;

        while *(_table.add(probe)) != std::ptr::null() {
            if equal(*(_table.add(probe)), item) {
                return *(_table.add(probe));
            }

            _colisions += 1;
            probe = (probe + increment) & (_size - 1);
        }

        *(_table.add(probe)) = item;
        return std::ptr::null();
    }

    pub const fn dump() {

        let mut field_width: i32;
        field_width = 0;
        
        {
            let mut i: i32 = _size - 1;
            while i >= 0 {
                if *(_table.add(i)) != std::ptr::null() {
                    if field_width < *(*(_table.add(i)))._selchars_length {
                        field_width = *(*(_table.add(i)))._selchars_length;
                    }
                }

                i -= 1;
            }
        }

        
        eprint!("\ndumping the hash table\ntotal available table slots = {}, total bytes = {}, total collisions = {}\nlocation, {:field_width$}, keyword\n", 
                        _size, _size * (std::mem::size_of_val(*_table) as u32), 
                        _collisions, "keysig");

        let mut i: i32 = _size - 1;
        while i >= 0 {
            if *(_table.add(i)) != std::ptr::null() {
                eprint!("{:>8}, ", i);
                if field_width > *(*(_table.add(i)))._selchars_length {
                    eprint!("{:>a$}", "", a = fieldwidth - *(*(_table.add(i)))._selchars_length);
                }
                let mut j: i32 = 0;
                while j < *(*(_table.add(i)))._selchars_length {
                    eprint!("{}", *(*(*(_table.add(i))).selchars.add(j)));
                    j += 1;
                }

                eprint!(", {:.a$}\n", *(*(_table.add(i)))._allchars, a = *(*(_table.add(i)))._allchars_length);
            } 
        }        

        eprint!("\nend dumping hash table\n\n");
    }

    #[inline]
    const fn equal(mut item1: *mut KeywordExt, mut item2: *mut KeywordExt) -> bool {

        if *item1._selchars_length != *item2._selchars_length {
            return false;
        }

        for i in 0..*item2._selchars_length {
            if *(*item1._selchars.add(i)) != *(*item2._selchars.add(i)) {
                return false;
            }
        }

        if _ignore_length == 0 && *item1._allchars_length != *item2._allchars_length {
            return false;
        }

        return true;

    }

}


impl Drop for Hash_Table {
    fn drop(&mut self) {
        std::mem::drop(_table);
    }
}
mod keyword;

struct Keyword {

    _cdr: *mut Keyword,
    _car: *mut Keyword_List 
}

impl Keyword_List {

    pub fn new(car: *mut Keyword) -> Keyword_List {
        Keyword{_cdr: std::ptr::null, _car: car}
    }


    pub const fn first() -> *mut Keyword {

        return _car;

    }

    pub fn rest() -> &*mut Keyword_List {

        return _cdr;

    }
    
}


struct KeywordExt_List {

    keyword_list: Keyword_List

}


impl KeywordExt_List {


    pub fn new(car: *mut KeywordExt) -> KeywordExt_List {

        Keyword_List::new(car)

    }



    pub const fn first() -> *mut KeywordExt {

        return keyword_list._car as *mut KeywordExt_List;

    }


    pub fn rest() -> &*mut KeywordExt {

        return *(&_cdr as *mut *mut KeywordExt_List); 

    } 

}

pub fn copy_list(list: *mut Keyword_List) -> *mut Keyword_List {

    let result: *mut Keyword_List;
    let lastp: *mut (*mut Keyword_List) = &result;

    while list != std::ptr::null() {
        
        let new_cons: *mut Keyword_List = Box::into_raw(Box::new((*list).first()));
        *lastp = new_cons;
        lastp = *(&new_cons).rest();
        list = (*list).rest();
    }

    *lastp = std::ptr::null();
    return result;
}


pub fn copy_extlist(list: *mut KeywordExt_List) -> *mut KeywordExt_List {
    return copy_list(list as *mut Keyword_List) as *mut Keyword_List;
}


pub fn delete_list(list: *mut Keyword_List) {

    while list != std::ptr::null() {
        let rest: *mut Keyword_List = (*list).rest();
        std::mem::drop(list);
        list = rest;
    }

}

type Keyword_Comparison = fn(keyword1: *mut Keyword, keyword2: *mut Keyword);

pub fn merge(list1: *mut Keyword_List, list2: *mut Keyword_List, less: Keyword_Comparison) -> *mut Keyword_List {

    let result: *mut Keyword_List;
    let resultp: *mut *mut Keyword_List = &result;

    while true {
        if !list1 {
            *resultp = list2;
            break;
        } 

        if !list2 {
            *resultp = list1;
            break;
        }

        if less((*list2).first(), (*list1).first()) {

            *resultp = list2;
            resultp = *(&list2).rest();
            list2 = list1;
            list1 = *resultp;
        } else {
            *resultp = list1;
            resultp = *(&list1).rest();
            list1 = *resultp;
        }  


    }

    return result;

}


pub fn mergesort_list(list: *mut Keyword_List, less: Keyword_Comparison) -> *mut Keyword_List {

    if list == std::ptr::null() || *list.rest() == std::ptr::null() {
        return list;
    } else {
        let middle: *mut Keyword_List;
        let temp: *mut Keyword_List;
        while true {
            temp = *temp.rest();
            if temp == std::ptr::null() {
                break;
            }
            temp = *temp.rest();
            middle = *middle.rest();
            if temp == std::ptr::null() {
                break;
            }
        }

        let right_half: *mut Keyword_List = *middle.rest();
        *middle.rest() = std::ptr::null;

        return merge(mergesort_list(list, less), mergesort_list(right_half, less), less);
    }

}

pub fn  mergesort_extlist(list: *mut KeywordExt_List, less: fn(keyword1: *mut Keyword, keyword2: *mut Keyword) -> bool) -> KeywordExt_List {

    return (mergesort_list(list as *mut Keyword_List, less as Keyword_Comparison) ) as *mut KeywordExt_List;

}








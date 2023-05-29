#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// wl_list rs wrapper
/// NOTE #[repr(C)] is mandatory
#[repr(C)]
pub struct List {
    // list is public for macros, however I would advise against
    // manually modifying it
    pub list: wl_list,
}

impl List {
    /// unsafe until init() member function is called
    pub fn new() -> List {
        unsafe {
            let mut nil = std::mem::MaybeUninit::<wl_list>::uninit();
            let list = wl_list {
                prev: nil.assume_init_mut(),
                next: nil.assume_init_mut(),
            };
            return List { list };
        }
    }

    #[inline]
    pub fn init(&mut self) {
        unsafe { wl_list_init(&mut self.list) };
    }

    #[inline]
    pub fn insert(&mut self, elm: &mut List) {
        unsafe { wl_list_insert(&mut self.list, &mut elm.list) };
    }

    #[inline]
    pub fn remove(&mut self, elm: &mut List) {
        unsafe { wl_list_remove(&mut elm.list) };
    }

    #[inline]
    #[must_use]
    pub fn length(&mut self) -> i32 {
        return unsafe { wl_list_length(&mut self.list) };
    }

    #[inline]
    #[must_use]
    pub fn empty(&mut self) -> i32 {
        return unsafe { wl_list_empty(&mut self.list) };
    }

    #[inline]
    pub fn insert_list(&mut self, other: &mut List) {
        unsafe { wl_list_insert_list(&mut self.list, &mut other.list) };
    }
}

/// __builtin_offset C macro clone
#[macro_export]
macro_rules! offset_of {
    ($struct: expr, $field: tt, $type: ty) => {{
        let dummy_ptr = &$struct as *const $type;
        let member_ptr = unsafe { ::std::ptr::addr_of!((*dummy_ptr).$field) };
        member_ptr as usize - dummy_ptr as usize
    }};
}

/// Behaves like wl_list_for_each and is perfectly compatable
/// Params
///     - $head: the head of the linked list
///     - $field: the field that is your linked list, 'link' by convension
///     - $type: typeof($head) NOTE wl_lists can only be one type to keep C compatablity
///     - $fn: a closure that takes in the list node's refernce as an arguement
///
/// [example]
///```
///    // NOTE the repr(C) for C struct layout in memory
///    #[repr(C)]
///    struct Message {
///        contents: String,
///        link: owl::List,
///    }
///    let mut head = Message {
///        // Rust structs must be initialized, hence the "NIL", however
///        // the fields of the 'head' listnode are never read (for wl_list compatbility)
///        contents: String::from("NIL"),
///        link: owl::List::new(),
///    };
///    let mut hello = Message {
///        contents: String::from("Hello,"),
///        link: owl::List::new(),
///    };
///    let mut world = Message {
///        contents: String::from(" World!"),
///        link: owl::List::new(),
///    };
///    // head.link must be initialized prior to use
///    // this is due to how rust changes stack memory in initializer function
///    // like '::new()'
///    head.link.init();
///    head.link.insert(&mut world.link);
///    head.link.insert(&mut hello.link);
///
///    let mut s: String = String::new();
///
///    owl::for_each!(head, link, Message, |x: &Message| {
///        s.push_str(&x.contents);
///    });
///
///    assert!(s == String::from("Hello, World!"));
///    println!("{s}");
///```
#[macro_export]
macro_rules! for_each {
    ($head: expr, $field: tt, $type: ty, $fn: expr) => {
        unsafe {
            let mut pos = ($head.$field.list.next as usize - owl::offset_of!($head, link, Message))
                as *const Message;

            while (&(*pos).link as *const owl::List != &$head.link) {
                $fn(&*pos);

                pos = ((*pos).link.list.next as usize - owl::offset_of!($head, link, Message))
                    as *const Message;
            }
        }
    };
}

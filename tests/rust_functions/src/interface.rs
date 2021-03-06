/* generated by rust_qt_binding_generator */
use libc::{c_char, c_ushort, c_int};
use std::slice;
use std::char::decode_utf16;

use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr::null;

use implementation::*;


pub enum QString {}

fn set_string_from_utf16(s: &mut String, str: *const c_ushort, len: c_int) {
    let utf16 = unsafe { slice::from_raw_parts(str, to_usize(len)) };
    let characters = decode_utf16(utf16.iter().cloned())
        .map(|r| r.unwrap());
    s.clear();
    s.extend(characters);
}



pub enum QByteArray {}


fn to_usize(n: c_int) -> usize {
    if n < 0 {
        panic!("Cannot cast {} to usize", n);
    }
    n as usize
}


fn to_c_int(n: usize) -> c_int {
    if n > c_int::max_value() as usize {
        panic!("Cannot cast {} to c_int", n);
    }
    n as c_int
}


pub struct PersonQObject {}

pub struct PersonEmitter {
    qobject: Arc<AtomicPtr<PersonQObject>>,
    user_name_changed: fn(*mut PersonQObject),
}

unsafe impl Send for PersonEmitter {}

impl PersonEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> PersonEmitter {
        PersonEmitter {
            qobject: self.qobject.clone(),
            user_name_changed: self.user_name_changed,
        }
    }
    fn clear(&self) {
        let n: *const PersonQObject = null();
        self.qobject.store(n as *mut PersonQObject, Ordering::SeqCst);
    }
    pub fn user_name_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.user_name_changed)(ptr);
        }
    }
}

pub trait PersonTrait {
    fn new(emit: PersonEmitter) -> Self;
    fn emit(&mut self) -> &mut PersonEmitter;
    fn user_name(&self) -> &str;
    fn set_user_name(&mut self, value: String);
    fn append(&mut self, suffix: String, amount: u32) -> ();
    fn double_name(&mut self) -> ();
    fn greet(&self, name: String) -> String;
    fn quote(&self, prefix: String, suffix: String) -> String;
    fn quote_bytes(&self, prefix: &[u8], suffix: &[u8]) -> Vec<u8>;
    fn vowels_in_name(&self) -> u8;
}

#[no_mangle]
pub extern "C" fn person_new(
    person: *mut PersonQObject,
    person_user_name_changed: fn(*mut PersonQObject),
) -> *mut Person {
    let person_emit = PersonEmitter {
        qobject: Arc::new(AtomicPtr::new(person)),
        user_name_changed: person_user_name_changed,
    };
    let d_person = Person::new(person_emit);
    Box::into_raw(Box::new(d_person))
}

#[no_mangle]
pub unsafe extern "C" fn person_free(ptr: *mut Person) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn person_user_name_get(
    ptr: *const Person,
    p: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let o = &*ptr;
    let v = o.user_name();
    let s: *const c_char = v.as_ptr() as (*const c_char);
    set(p, s, to_c_int(v.len()));
}

#[no_mangle]
pub unsafe extern "C" fn person_user_name_set(ptr: *mut Person, v: *const c_ushort, len: c_int) {
    let o = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_user_name(s);
}

#[no_mangle]
pub unsafe extern "C" fn person_append(ptr: *mut Person, suffix_str: *const c_ushort, suffix_len: c_int, amount: u32) {
    let mut suffix = String::new();
    set_string_from_utf16(&mut suffix, suffix_str, suffix_len);
    let o = &mut *ptr;
    o.append(suffix, amount)
}

#[no_mangle]
pub unsafe extern "C" fn person_double_name(ptr: *mut Person) {
    let o = &mut *ptr;
    o.double_name()
}

#[no_mangle]
pub unsafe extern "C" fn person_greet(ptr: *const Person, name_str: *const c_ushort, name_len: c_int, d: *mut QString, set: fn(*mut QString, str: *const c_char, len: c_int)) {
    let mut name = String::new();
    set_string_from_utf16(&mut name, name_str, name_len);
    let o = &*ptr;
    let r = o.greet(name);
    let s: *const c_char = r.as_ptr() as (*const c_char);
    set(d, s, r.len() as i32);
}

#[no_mangle]
pub unsafe extern "C" fn person_quote(ptr: *const Person, prefix_str: *const c_ushort, prefix_len: c_int, suffix_str: *const c_ushort, suffix_len: c_int, d: *mut QString, set: fn(*mut QString, str: *const c_char, len: c_int)) {
    let mut prefix = String::new();
    set_string_from_utf16(&mut prefix, prefix_str, prefix_len);
    let mut suffix = String::new();
    set_string_from_utf16(&mut suffix, suffix_str, suffix_len);
    let o = &*ptr;
    let r = o.quote(prefix, suffix);
    let s: *const c_char = r.as_ptr() as (*const c_char);
    set(d, s, r.len() as i32);
}

#[no_mangle]
pub unsafe extern "C" fn person_quote_bytes(ptr: *const Person, prefix_str: *const c_char, prefix_len: c_int, suffix_str: *const c_char, suffix_len: c_int, d: *mut QByteArray, set: fn(*mut QByteArray, str: *const c_char, len: c_int)) {
    let prefix = { slice::from_raw_parts(prefix_str as *const u8, to_usize(prefix_len)) };
    let suffix = { slice::from_raw_parts(suffix_str as *const u8, to_usize(suffix_len)) };
    let o = &*ptr;
    let r = o.quote_bytes(prefix, suffix);
    let s: *const c_char = r.as_ptr() as (*const c_char);
    set(d, s, r.len() as i32);
}

#[no_mangle]
pub unsafe extern "C" fn person_vowels_in_name(ptr: *const Person) -> u8 {
    let o = &*ptr;
    o.vowels_in_name()
}

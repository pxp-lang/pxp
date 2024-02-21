#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[link(name = "wrapper")]
extern "C" {
    
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#[allow(dead_code)]
pub mod wasi {
    #[allow(dead_code)]
    pub mod random {
        #[allow(dead_code, clippy::all)]
        pub mod random {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            /// Return `len` cryptographically-secure random or pseudo-random bytes.
            ///
            /// This function must produce data at least as cryptographically secure and
            /// fast as an adequately seeded cryptographically-secure pseudo-random
            /// number generator (CSPRNG). It must not block, from the perspective of
            /// the calling program, under any circumstances, including on the first
            /// request and on requests for numbers of bytes. The returned data must
            /// always be unpredictable.
            ///
            /// This function must always return fresh data. Deterministic environments
            /// must omit this function, rather than implementing it with deterministic
            /// data.
            pub fn get_random_bytes(len: u64) -> _rt::Vec<u8> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/random@0.2.0")]
                    extern "C" {
                        #[link_name = "get-random-bytes"]
                        fn wit_import(_: i64, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(_rt::as_i64(&len), ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let len3 = l2;
                    _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Return a cryptographically-secure random or pseudo-random `u64` value.
            ///
            /// This function returns the same type of data as `get-random-bytes`,
            /// represented as a `u64`.
            pub fn get_random_u64() -> u64 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/random@0.2.0")]
                    extern "C" {
                        #[link_name = "get-random-u64"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod insecure {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            /// Return `len` insecure pseudo-random bytes.
            ///
            /// This function is not cryptographically secure. Do not use it for
            /// anything related to security.
            ///
            /// There are no requirements on the values of the returned bytes, however
            /// implementations are encouraged to return evenly distributed values with
            /// a long period.
            pub fn get_insecure_random_bytes(len: u64) -> _rt::Vec<u8> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/insecure@0.2.0")]
                    extern "C" {
                        #[link_name = "get-insecure-random-bytes"]
                        fn wit_import(_: i64, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(_rt::as_i64(&len), ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let len3 = l2;
                    _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Return an insecure pseudo-random `u64` value.
            ///
            /// This function returns the same type of pseudo-random data as
            /// `get-insecure-random-bytes`, represented as a `u64`.
            pub fn get_insecure_random_u64() -> u64 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/insecure@0.2.0")]
                    extern "C" {
                        #[link_name = "get-insecure-random-u64"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod insecure_seed {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            #[allow(unused_unsafe, clippy::all)]
            /// Return a 128-bit value that may contain a pseudo-random value.
            ///
            /// The returned value is not required to be computed from a CSPRNG, and may
            /// even be entirely deterministic. Host implementations are encouraged to
            /// provide pseudo-random values to any program exposed to
            /// attacker-controlled content, to enable DoS protection built into many
            /// languages' hash-map implementations.
            ///
            /// This function is intended to only be called once, by a source language
            /// to initialize Denial Of Service (DoS) protection in its hash-map
            /// implementation.
            ///
            /// # Expected future evolution
            ///
            /// This will likely be changed to a value import, to prevent it from being
            /// called multiple times and potentially used for purposes other than DoS
            /// protection.
            pub fn insecure_seed() -> (u64, u64) {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/insecure-seed@0.2.0")]
                    extern "C" {
                        #[link_name = "insecure-seed"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i64>();
                    (l1 as u64, l2 as u64)
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod even_odd {
            #[allow(dead_code, clippy::all)]
            pub mod random_demo {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_is_even_cabi<T: Guest>() -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::is_even();
                    match result0 {
                        true => 1,
                        false => 0,
                    }
                }
                pub trait Guest {
                    fn is_even() -> bool;
                }
                #[doc(hidden)]
                macro_rules! __export_component_even_odd_random_demo_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:even-odd/random-demo@0.1.0#is-even"] unsafe extern "C"
                        fn export_is_even() -> i32 { $($path_to_types)*::
                        _export_is_even_cabi::<$ty > () } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_even_odd_random_demo_0_1_0_cabi;
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::vec::Vec;
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_example_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::component::even_odd::random_demo::__export_component_even_odd_random_demo_0_1_0_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::component::even_odd::random_demo);
    };
}
#[doc(inline)]
pub(crate) use __export_example_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:component:even-odd@0.1.0:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 497] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xf3\x02\x01A\x02\x01\
A\x08\x01B\x05\x01p}\x01@\x01\x03lenw\0\0\x04\0\x10get-random-bytes\x01\x01\x01@\
\0\0w\x04\0\x0eget-random-u64\x01\x02\x03\x01\x18wasi:random/random@0.2.0\x05\0\x01\
B\x05\x01p}\x01@\x01\x03lenw\0\0\x04\0\x19get-insecure-random-bytes\x01\x01\x01@\
\0\0w\x04\0\x17get-insecure-random-u64\x01\x02\x03\x01\x1awasi:random/insecure@0\
.2.0\x05\x01\x01B\x03\x01o\x02ww\x01@\0\0\0\x04\0\x0dinsecure-seed\x01\x01\x03\x01\
\x1fwasi:random/insecure-seed@0.2.0\x05\x02\x01B\x02\x01@\0\0\x7f\x04\0\x07is-ev\
en\x01\0\x04\x01$component:even-odd/random-demo@0.1.0\x05\x03\x04\x01\x20compone\
nt:even-odd/example@0.1.0\x04\0\x0b\x0d\x01\0\x07example\x03\0\0\0G\x09producers\
\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.0\x10wit-bindgen-rust\x060.31\
.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}

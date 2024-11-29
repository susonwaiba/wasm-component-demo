#[allow(dead_code)]
pub mod component {
    #[allow(dead_code)]
    pub mod calculator {
        #[allow(dead_code, clippy::all)]
        pub mod calculate {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            #[allow(unused_unsafe, clippy::all)]
            pub fn eval_expression(expr: &str) -> u32 {
                unsafe {
                    let vec0 = expr;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "component:calculator/calculate")]
                    extern "C" {
                        #[link_name = "eval-expression"]
                        fn wit_import(_: *mut u8, _: usize) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(ptr0.cast_mut(), len0);
                    ret as u32
                }
            }
        }
    }
}
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.30.0:cli-app:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 230] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07i\x01A\x02\x01A\x02\x01\
B\x02\x01@\x01\x04exprs\0y\x04\0\x0feval-expression\x01\0\x03\x01\x1ecomponent:c\
alculator/calculate\x05\0\x04\x01\x19component:cli-app/cli-app\x04\0\x0b\x0d\x01\
\0\x07cli-app\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x07\
0.215.0\x10wit-bindgen-rust\x060.30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}

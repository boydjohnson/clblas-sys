extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=clBLAS");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .blocklist_item("pthread.*")
        .rustfmt_bindings(true)
        .opaque_type("cl_char16__bindgen_ty_3")
        .opaque_type("cl_float4__bindgen_ty_3")
        .opaque_type("cl_int4__bindgen_ty_3")
        .opaque_type("cl_short8__bindgen_ty_3")
        .opaque_type("cl_uchar16__bindgen_ty_3")
        .opaque_type("cl_uint4__bindgen_ty_3")
        .opaque_type("cl_ushort8__bindgen_ty_3")
        .opaque_type("cl_char8")
        .opaque_type("cl_float2")
        .opaque_type("cl_int2")
        .opaque_type("cl_short4")
        .opaque_type("cl_uchar8")
        .opaque_type("cl_uint2")
        .opaque_type("cl_ushort4")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

extern crate bindgen;

use std::{env, path::PathBuf};

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
        .blocklist_type("cl_command_queue")
        .raw_line("use cl_sys::cl_command_queue;")
        .blocklist_type("cl_context")
        .raw_line("use cl_sys::cl_context;")
        .blocklist_type("cl_context_properties")
        .raw_line("use cl_sys::cl_context_properties;")
        .blocklist_type("cl_program")
        .raw_line("use cl_sys::cl_program;")
        .blocklist_type("cl_device_id")
        .raw_line("use cl_sys::cl_device_id;")
        .blocklist_type("cl_device_type")
        .raw_line("use cl_sys::cl_device_type;")
        .blocklist_type("cl_kernel")
        .raw_line("use cl_sys::cl_kernel;")
        .blocklist_type("cl_kernel_arg_info")
        .raw_line("use cl_sys::cl_kernel_arg_info;")
        .blocklist_type("cl_command_queue_properties")
        .raw_line("use cl_sys::cl_command_queue_properties;")
        .blocklist_type("cl_event")
        .raw_line("use cl_sys::cl_event;")
        .blocklist_type("cl_mem")
        .raw_line("use cl_sys::cl_mem;")
        .blocklist_type("cl_mem_flags")
        .raw_line("use cl_sys::cl_mem_flags;")
        .blocklist_type("cl_device_info")
        .raw_line("use cl_sys::cl_device_info;")
        .blocklist_type("cl_command_queue_info")
        .raw_line("use cl_sys::cl_command_queue_info;")
        .blocklist_type("cl_platform_id")
        .raw_line("use cl_sys::cl_platform_id;")
        .blocklist_type("cl_image_info")
        .raw_line("use cl_sys::cl_image_info;")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

use crate::spec::{LinkArgs, LinkerFlavor, TargetOptions};
use std::default::Default;

pub fn opts() -> TargetOptions {
    let mut args = LinkArgs::new();
    // GCC
    // pre_link_args.insert(LinkerFlavor::Gcc, vec![
    //         // Tell GCC to avoid linker plugins, because we are not bundling
    //         // them with Windows installer, and Rust does its own LTO anyways.
    //         "-fno-use-linker-plugin".to_string(),

    //         // Always enable DEP (NX bit) when it is available
    //         "-Wl,--nxcompat".to_string(),

    //         // Do not use the standard system startup files or libraries when linking
    //         "-nostdlib".to_string(),
    //     ]);

    // MSVC
    args.insert(LinkerFlavor::Msvc,
                vec!["/NOLOGO".to_string(),
                     "/NXCOMPAT".to_string()]);

    // GCC
    // let mut late_link_args = LinkArgs::new();
    // late_link_args.insert(LinkerFlavor::Gcc, vec![
    //     //"-lwinstorecompat".to_string(),
    //     //"-lmingwex".to_string(),
    //     //"-lwinstorecompat".to_string(),
    //     "-lwinstorecompat".to_string(),
    //     "-lruntimeobject".to_string(),
    //     "-lsynchronization".to_string(),
    //     "-lvcruntime140_app".to_string(),
    //     "-lucrt".to_string(),
    //     "-lwindowsapp".to_string(),
    //     "-lmingwex".to_string(),
    //     "-lmingw32".to_string(),
    // ]);

    TargetOptions {
        // FIXME(#13846) this should be enabled for windows
        function_sections: true,
        // linker: Some("gcc".to_string()),
        linker: Some("link.exe".to_string()),
        dynamic_linking: true,
        executables: false,
        dll_prefix: String::new(),
        dll_suffix: ".dll".to_string(),
        exe_suffix: ".exe".to_string(),
        staticlib_prefix: "lib".to_string(),
        staticlib_suffix: ".a".to_string(),
        no_default_libraries: true,
        target_family: Some("windows".to_string()),
        is_like_windows: true,
        allows_weak_linkage: false,
        pre_link_args: args,
        
        // custom_unwind_resume: true,
        abi_return_struct_as_int: true,
        emit_debug_gdb_scripts: false,
        requires_uwtable: true,
        
        crt_static_allows_dylibs: true,
        crt_static_respected: true,

        .. Default::default()
    }
}

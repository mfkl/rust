use crate::spec::{LinkArgs, LinkerFlavor, TargetOptions};
use std::default::Default;

pub fn opts() -> TargetOptions {
    let mut pre_link_args = LinkArgs::new();
    args.insert(LinkerFlavor::Msvc,
                vec!["/NOLOGO".to_string(),
                     "/NXCOMPAT".to_string()]);

    TargetOptions {
        function_sections: true,
//        linker: Some("link.exe".to_string()),
        dynamic_linking: true,
        executables: false,
        dll_prefix: String::new(),
        dll_suffix: ".dll".to_string(),
        exe_suffix: ".exe".to_string(),
        staticlib_prefix: String::new(),
        staticlib_suffix: ".lib".to_string(),
        target_family: Some("windows".to_string()),
        is_like_windows: true,
        is_like_msvc: true,
        pre_link_args: pre_link_args,
        crt_static_allows_dylibs: true,
        crt_static_respected: true,
        abi_return_struct_as_int: true,
        emit_debug_gdb_scripts: false,
        requires_uwtable: true,

        .. Default::default()
    }
}

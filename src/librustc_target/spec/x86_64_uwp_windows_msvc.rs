use crate::spec::{LinkerFlavor, Target, TargetResult};

pub fn target() -> TargetResult {
    let mut base = super::windows_uwp_msvc_base::opts();
    base.cpu = "x86-64".to_string();
    base.max_atomic_width = Some(64);
    base.has_elf_tls = true;
    base.pre_link_args
        .get_mut(&LinkerFlavor::Msvc).unwrap().push("/LIBPATH:C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\lib\\store\\amd64".to_string());

    Ok(Target {
        llvm_target: "x86_64-pc-windows-msvc".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "e-m:w-i64:64-f80:128-n8:16:32:64-S128".to_string(),
        arch: "x86_64".to_string(),
        target_os: "windows".to_string(),
        target_env: "msvc".to_string(),
        target_vendor: "uwp".to_string(),
        linker_flavor: LinkerFlavor::Msvc,
        options: base,
    })
}
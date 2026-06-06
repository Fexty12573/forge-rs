//! Regenerates `forge-sys/src/imgui_bindings.rs` from the vendored cimgui submodule.
//!     cargo run --manifest-path tools/imgui-bindgen/Cargo.toml [-- <target>]

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use regex::Regex;

const DEFAULT_TARGET: &str = "armv7a-none-eabi";

/// Merge enum typedefs and actual enums into one
fn fold_twin_types(code: &str) -> String {
    let type_re = Regex::new(r"(?m)^pub (?:enum|struct) (Im[A-Za-z0-9]+)_\b").unwrap();
    let bases: BTreeSet<String> = type_re.captures_iter(code).map(|c| c[1].to_string()).collect();

    let mut out = code.to_string();
    for base in &bases {
        let typedef = Regex::new(&format!(r"(?m)^pub type {base} = ::core::ffi::c_(?:int|uint);\n")).unwrap();
        out = typedef.replace(&out, "").into_owned();

        let occ = Regex::new(&format!(r"\b{base}_\b")).unwrap();
        out = occ.replace_all(&out, regex::NoExpand(base)).into_owned();
    }
    out
}

/// Strips the enum's own name from its variants, so cimgui's
/// `ImGuiInputTextFlags_None` becomes `ImGuiInputTextFlags::None` instead of
/// `ImGuiInputTextFlags_::ImGuiInputTextFlags_None`.
#[derive(Debug)]
struct StripEnumPrefix;

impl bindgen::callbacks::ParseCallbacks for StripEnumPrefix {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        variant: &str,
        _value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        let prefix = enum_name?.trim_start_matches("enum ").to_owned();
        let prefix = if prefix.ends_with('_') { prefix } else { prefix + "_" };
        let stripped = variant.strip_prefix(&prefix)?;
        if stripped.is_empty() {
            return None; // keep original if stripping would leave nothing
        }

        if stripped.as_bytes()[0].is_ascii_digit() {
            Some(format!("_{stripped}"))
        } else {
            Some(stripped.to_string())
        }
    }
}

fn main() {
    let target = std::env::args().nth(1).unwrap_or_else(|| DEFAULT_TARGET.to_string());

    // tools/imgui-bindgen -> repo root
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let imgui_dir = repo_root.join("forge-sys").join("imgui");
    let cimgui_dir = imgui_dir.join("external").join("cimgui");
    let wrapper = imgui_dir.join("wrapper.h");
    let shim_dir = imgui_dir.join("shim");
    let out_file = repo_root.join("forge-sys").join("src").join("imgui_bindings.rs");

    assert!(
        cimgui_dir.join("cimgui.h").is_file(),
        "cimgui.h not found at {} — run `git submodule update --init`",
        cimgui_dir.display()
    );

    let bindings = bindgen::Builder::default()
        .header(wrapper.to_str().unwrap())
        .clang_arg(format!("--target={target}"))
        .clang_arg("-nostdlibinc")
        .clang_arg(format!("-isystem{}", shim_dir.display()))
        .clang_arg(format!("-I{}", cimgui_dir.display()))
        .use_core()
        .layout_tests(false)
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .merge_extern_blocks(true)
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
        // All flag-likes as bitfields so `A | B` works: ImGuiWindowFlags_,
        // ImDrawFlags_, ImFontFlags_, ImGuiButtonFlagsPrivate_, ...
        .bitfield_enum(r"^Im\w*Flags\w*_$")
        .allowlist_file(r".*cimgui\.h")
        .prepend_enum_name(false)
        .parse_callbacks(Box::new(StripEnumPrefix))
        .generate()
        .expect("failed to generate cimgui bindings");

    bindings.write_to_file(&out_file).expect("failed to write imgui_bindings.rs");

    let folded = fold_twin_types(&fs::read_to_string(&out_file).unwrap());
    fs::write(&out_file, folded).expect("failed to write imgui_bindings.rs");

    println!("regenerated {} for target {target}", out_file.display());
}

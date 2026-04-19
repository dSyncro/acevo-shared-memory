use std::path::PathBuf;

const WRAPPER_HPP: &str = "src/bindings/source/wrapper.hpp";
const BINDINGS_FILE: &str = "bindings.rs";

fn main() {
    println!("cargo:rerun-if-changed={WRAPPER_HPP}");

    let out_path = out_path();
    generate_bindings(&out_path);
    patch_serde(&out_path);
}

fn out_path() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap()).join(BINDINGS_FILE)
}

fn generate_bindings(out_path: &PathBuf) {
    bindgen::Builder::default()
        .header(WRAPPER_HPP)
        .enable_cxx_namespaces()
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path)
        .expect("Couldn't write bindings");
}

/// Post-processes the generated bindings to add feature-gated serde support:
///
/// - Injects `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]`
///   before every `pub struct`.
/// - Injects `#[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]`
///   before any field whose array size exceeds 32 (serde's fixed-impl limit),
///   covering the large C char arrays present in the protocol structs.
fn patch_serde(out_path: &PathBuf) {
    let content = std::fs::read_to_string(out_path).expect("Could not read bindings");

    let patched = content
        .lines()
        .flat_map(patch_line)
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write(out_path, patched).expect("Could not write patched bindings");
}

fn patch_line(line: &str) -> Vec<String> {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];

    if trimmed.starts_with("pub struct ") {
        let serde_attr = format!(
            "{indent}#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]"
        );
        return vec![serde_attr, line.to_string()];
    }

    if trimmed.starts_with("pub ") && array_size(trimmed) > 32 {
        let serde_attr =
            format!("{indent}#[cfg_attr(feature = \"serde\", serde(with = \"serde_arrays\"))]");
        return vec![serde_attr, line.to_string()];
    }

    vec![line.to_string()]
}

/// Extracts the element count from a Rust array type `[Type; N]` or `[Type; Nusize]`.
/// Returns 0 if the line does not contain an array type.
fn array_size(field_line: &str) -> usize {
    if let (Some(open), Some(semi), Some(close)) = (
        field_line.rfind('['),
        field_line.rfind(';'),
        field_line.rfind(']'),
    ) && open < semi
        && semi < close
    {
        return field_line[(semi + 1)..close]
            .trim()
            .trim_end_matches("usize")
            .trim()
            .parse()
            .unwrap_or(0);
    }

    0
}

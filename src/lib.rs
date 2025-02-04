#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod bindings;
mod gen;

use bindgen::Builder;
use syn::{parse::Error as ParseError, File, ForeignItemFn};

pub trait BindingStrategy {
    fn should_include(&self, item: &ForeignItemFn) -> bool;
}

pub fn generate_bindings<S>(
    builder: Builder,
    strategy: &S,
) -> Result<File, Error>
where
    S: BindingStrategy,
{
    let raw_bindings = builder
        .generate()
        .map_err(|_| Error::BindgenFailed)?
        .to_string();
    let mut file: File = syn::parse_str(&raw_bindings)?;

    let bindings = bindings::extract_raw_bindings(&mut file, strategy);
    gen::append_new_bindings(&mut file.items, bindings);

    Ok(file)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Bindgen was unable to generate bindings")]
    BindgenFailed,
    #[error("Unable to parse the bindings emitted by bindgen")]
    Parse(#[from] ParseError),
}

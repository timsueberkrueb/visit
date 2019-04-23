// Diagnostics shim borrowed from swirl [1]
// Necessary until procedural macro diagnostics (RFC 1566, [2]) stabilizes.
//
// [1]: https://github.com/sgrif/swirl/blob/master/swirl_proc_macro/src/diagnostic_shim.rs
// [2]: https://github.com/rust-lang/rust/issues/54140

use proc_macro2::{Span, TokenStream};

pub trait DiagnosticShim {
    fn error<T: Into<String>>(self, msg: T) -> Diagnostic;
}

#[cfg(feature = "nightly")]
impl DiagnosticShim for Span {
    fn error<T: Into<String>>(self, msg: T) -> Diagnostic {
        self.unstable().error(self, msg)
    }
}

#[cfg(not(feature = "nightly"))]
impl DiagnosticShim for Span {
    fn error<T: Into<String>>(self, msg: T) -> Diagnostic {
        Diagnostic::error(self, msg)
    }
}

#[cfg(feature = "nightly")]
pub use proc_macro::Diagnostic;

#[cfg(not(feature = "nightly"))]
pub struct Diagnostic {
    span: Span,
    message: String,
}

#[cfg(not(feature = "nightly"))]
impl Diagnostic {
    fn error<T: Into<String>>(span: Span, msg: T) -> Self {
        Diagnostic {
            span,
            message: msg.into(),
        }
    }
}

pub trait DiagnosticExt {
    fn to_compile_error(self) -> TokenStream;
}

#[cfg(feature = "nightly")]
impl DiagnosticExt for Diagnostic {
    fn to_compile_error(self) -> TokenStream {
        self.emit();
        "".parse().unwrap()
    }
}

#[cfg(not(feature = "nightly"))]
impl DiagnosticExt for Diagnostic {
    fn to_compile_error(self) -> TokenStream {
        syn::Error::new(self.span, self.message).to_compile_error()
    }
}

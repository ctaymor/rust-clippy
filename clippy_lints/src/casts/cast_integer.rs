use clippy_utils::msrvs::{self, Msrv};
use rustc_lint::{LateContext, LintContext};

use super::CAST_INTEGER;

// TODO: Adjust the parameters as necessary
pub(super) fn check(cx: &LateContext, msrv: &Msrv) {
    if !msrv.meets(todo!("Add a new entry in `clippy_utils/src/msrvs`")) {
        return;
    }
    todo!();
}

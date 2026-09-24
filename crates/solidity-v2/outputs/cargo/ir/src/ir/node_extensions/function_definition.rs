use std::ops::Range;

use crate::ir;

impl ir::FunctionDefinitionStruct {
    /// Whether the function is declared `public` or `external`.
    pub fn is_externally_visible(&self) -> bool {
        matches!(
            self.attributes.visibility,
            ir::FunctionVisibility::Public | ir::FunctionVisibility::External
        )
    }

    /// Whether the function is dispatched by selector: an externally visible
    /// `Regular` function (a constructor, fallback or receive has no name to
    /// select on, and a modifier is not called). Mirrors solc's
    /// `isPartOfExternalInterface`.
    pub fn is_part_of_external_interface(&self) -> bool {
        self.is_externally_visible() && matches!(self.kind, ir::FunctionKind::Regular)
    }

    /// The text range of the function's signature, ie. everything but its body.
    ///
    /// Falls back to the full range for bodyless declarations (eg. functions in
    /// interfaces or abstract contracts).
    ///
    pub fn signature_text_range(&self) -> Range<usize> {
        // TODO: This shouldn't use the start of the body, but right now
        // the IR doesn't have enough information to compute the
        // actual range.
        let end = match &self.body {
            Some(body) => body.range.start,
            None => self.range.end,
        };
        self.range.start..end
    }
}

#[allow(unused)] // TEMPORARY while building
use crate::{Content, Parser};

pub(super) fn apply_macros(content: &mut Content<'_>, parser: &Parser) {
    let text = content.rendered();
    let found_square_bracket = text.contains('[');
    let found_colon = text.contains(':');
    let found_macroish = found_square_bracket && found_colon;
    let found_macroish_short = found_macroish && text.contains(":[");

    // TO DO (#262): Implement extensions that can define macros.
    // Port Ruby Asciidoctor's implementation from
    // https://github.com/asciidoctor/asciidoctor/blob/main/lib/asciidoctor/substitutors.rb#L306-L347.

    // TO DO (#263): Implement `kbd:` and `btn:` macros.
    // Port Ruby Asciidoctor's implementation from
    // https://github.com/asciidoctor/asciidoctor/blob/main/lib/asciidoctor/substitutors.rb#L349-L377.

    if found_macroish && (text.contains("image:") || text.contains("icon:")) {
        todo!("Image and icon macros");
        // Port Ruby Asciidoctor's implementation from lines 417..437.
    }

    if (text.contains("((") && text.contains("))"))
        || (found_macroish_short && text.contains("dexterm"))
    {
        todo!("Index term macro");
        // Port Ruby Asciidoctor's implementation from lines 439..536.
    }

    if found_colon && text.contains("://") {
        todo!("URL macro");
        // Port Ruby Asciidoctor's implementation from lines 538..634.
    }

    if found_macroish && (text.contains("link:") || text.contains("ilto:")) {
        todo!("Link macro");
        // Port Ruby Asciidoctor's implementation from lines 636..704.
    }

    if text.contains('@') {
        todo!("Maybe found email macro");
        // Port Ruby Asciidoctor's implementation from lines 706..717.
    }

    if found_square_bracket && false {
        // 'false' should be replaced with @context == :list_item && @parent.style ==
        // 'bibliography'.
        todo!("Port bibliography reference macro");
        // Port Ruby Asciidoctor's implementation from lines 719..721.
    }

    if (found_square_bracket && text.contains("[[")) || (found_macroish && text.contains("or:")) {
        todo!("Port inline anchor macro");
        // Port Ruby Asciidoctor's implementation from lines 723..739.
    }

    if (text.contains('&') && text.contains(";&l") || (found_macroish && text.contains("xref:"))) {
        todo!("Port cross-reference macro");
        // Port Ruby Asciidoctor's implementation from lines 742..840.
    }

    if found_macroish && text.contains("tnote") {
        todo!("Port footnote macro");
        // Port Ruby Asciidoctor's implementation from lines 842..884.
    }
}

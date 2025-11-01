use crate::tests::prelude::*;

track_file!("docs/modules/sections/pages/special-section-titles.adoc");

// Treating the entire file as non-normative because we don't support `doctype`.

non_normative!(
    r#"
= Hide Special Section Titles

If supported by the converter, the title of a xref:styles.adoc[special section], such as the Dedication, can be turned off by setting the `notitle` option (e.g., `%notitle` or `opts=notitle`) (previously `untitled`) on the section.

----
[dedication%notitle]
== Dedication

include::example$dedication.adoc[tag=body]
----

Although the title is hidden in the output document, it still needs to be specified in the AsciiDoc source for the purpose of referencing.
The title will be used as the reftext of a cross reference, just as with any section.
"#
);

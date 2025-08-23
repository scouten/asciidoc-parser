use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/attributes/pages/inline-attribute-entries.adoc");

// IMPORTANT: It is unlikely that the asciidoc-parser crate will ever support
// inline attribute entries given the "CAUTION" paragraph included here and
// other signs that the core AsciiDoc team are moving to remove this syntax from
// the language.

non_normative!(
    r#"
= Inline Attribute Entries

An attribute reference can be used to set or unset an attribute inline as an alternative to a dedicated attribute entry line.
This mechanism allows you to set or unset an attribute in places where attribute entries lines are not permitted, such as in a normal table cell or a list item.

CAUTION: You're strongly discouraged from using inline attribute entries unless you understand their limitations or they are a last resort for fulfilling a use case.
It's very likely that this functionality will be removed from the AsciiDoc language since its behavior is difficult to define.

Attributes can be defined inline using the following notation:

----
{set:name:value}
----

The value segment is optional.
If absent, the value defaults to empty string.
In that case, the notation is reduced to:

----
{set:name}
----

If you add a `!` character after the name to unset the attribute instead:

----
{set:name!}
----

Here's an example that uses an inline attribute entry to set the `sourcedir` attribute to the value `src/main/java`.

----
{set:sourcedir:src/main/java}
----

This assignment is effectively the same as:

----
:sourcedir: src/main/java
----

However, it's important to understand that inline attribute assignments are processed in a different phase than attribute entry lines.
Inline attribute entries are processed when attribute references are replaced, as part of the attributes substitution.
Therefore, the result of the assignment is only available to attribute references that follow it.
These assignments are not visible in the document model after the document has been loaded.

"#
);

use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/macros/pages/icons.adoc");

non_normative!(
    r#"
= Icons

Icons are a useful way to communicate information visually while at the same time eliminating text that can distract from the primary text.
Icons also have the benefit of adding some flair to your document.
In AsciiDoc, there are numerous ways to embellish the output of your document with icons (for backends that support this feature).

For some elements, icons are added automatically by the processor when enabled, such as the admonition icons.
You can also add icons directly to the content using special markup.

This section shows you how to enable icons, covers the various icon modes, and introduces you to the icon macro for adding custom icons to your content.

= Icons

Icons are a useful way to communicate information visually while at the same time eliminating text that can distract from the primary text.
Icons also have the benefit of adding some flair to your document.
In AsciiDoc, there are numerous ways to embellish the output of your document with icons (for backends that support this feature).

For some elements, icons are added automatically by the processor when enabled, such as the admonition icons.
You can also add icons directly to the content using special markup.

This section shows you how to enable icons, covers the various icon modes, and introduces you to the icon macro for adding custom icons to your content.

[#icons-attribute]
== Enable icons

There are three icon modes: text, image, and font.

The inclusion of icons in the output is controlled using the `icons` document attribute.
By default, this attribute is not set.
As a result, all icons are displayed as text.
In the text icon mode, icons are effectively disabled.

To enable icons, set the `icons` attribute in the document header.

[source]
----
= Document Title
:icons:
----

Valid values for the `icons` attribute are as follows:

image (or empty)::
Icons resolve to image files in the directory specified by the `iconsdir` attribute.

font::
Icons are loaded from an icon font (like Font Awesome).
Not all backends support this mode, such as DocBook.

== Where icons are used

Setting the `icon` attribute turns on icons in the following locations:

* The admonition label is replaced with an icon (i.e., admonition icons)
* The icon macro
* Callout numbers

The following pages will cover these icon modes in more depth.
"#
);

use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/macros/pages/images-directory.adoc");

non_normative!(
    r#"
= Set the Images Directory

The path to the location of the image catalog is controlled by the `imagesdir` attribute.

== imagesdir attribute syntax

`imagesdir` is a document attribute.
Its value is automatically added to the beginning of every image macro target.
The resolved location of a image is: `<value-of-imagesdir> + <image-macro-target>`.
Therefore, you never need to reference this attribute in an image macro.
You only need to set it in your document header.

.Incorrect
[source]
----
image::{imagesdir}/name-of-image.png[]
----

.Correct
[source]
----
image::name-of-image.png[]
----

The value of `imagesdir` can be an absolute path, relative path or URL.
By default, the `imagesdir` value is empty.
That means the images are resolved relative to the document.
If an image macro's target is an absolute path or URL, the value of `imagesdir` is not added to the target path.

The benefit of the processor adding the value of `imagesdir` to the start of all image targets is that you can globally control the folder where images are located per converter.
We refer to this folder as the image catalog.
Since different output formats require the images to be stored in different locations, this attribute makes it possible to accommodate many different scenarios.

We recommend relying on `imagesdir` when defining the target of your image to avoid hard-coding that common path in every single image macro.
Always think about where the image is relative to the image catalog.

TIP: You can set the `imagesdir` attribute in multiple places in your document, as long as it is not locked by the API.
This technique is useful if you store images for different parts, chapters, or sections of your document in different locations.
"#
);

use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/ROOT/pages/asciidoc-vs-markdown.adoc");

non_normative!(
    r#"
= Compare AsciiDoc to Markdown
:description: A brief, side-by-side comparison of AsciiDoc and Markdown.

The most compelling reason to choose a lightweight markup language for writing is to minimize the number of technical concepts an author must grasp in order to be immediately productive.
In other words, the goal is to be able to _write without friction_.
While that's certainly the goal of both AsciiDoc and Markdown, and both are very approachable for newcomers, this page explores why AsciiDoc is a more suitable alternative to Markdown as your content grows and evolves.

== Starting with Markdown

The most prevalent lightweight markup language is Markdown.
(At least, Markdown is what you call it at first).
The main advantage of Markdown lies in its primitive syntax: its manual and cheatsheet are one and the same.
But this advantage is also its greatest weakness.

As soon as authors need something slightly more complex than basic prose (e.g., tables, cross references, footnotes, embedded YouTube videos, etc.), they find themselves resorting to embedded HTML or seeking out more feature-rich implementations.
Markdown has become a maze of different implementations, termed "`flavors`", which make a universal definition evasive.

NOTE: The IETF has declared "`there is no such thing as "invalid" Markdown.`"
See https://tools.ietf.org/html/rfc7763#section-1.1[This Is Markdown! Or: Markup and Its Discontents^].

Here's how the story inevitably goes.
You start out with Markdown.
Then it's Markdown + X.
Then Markdown + X + Y.
And down the rabbit hole you go.
What's worse, X and Y often require you to sprinkle in HTML, unnecessarily coupling content with presentation and wrecking portability.
Your instinct to choose Markdown is good.
There are just better options.

== Graduating to AsciiDoc

AsciiDoc presents a more sound alternative.
The AsciiDoc syntax is more concise than (or at least as concise as) Markdown.
At the same time, AsciiDoc offers power and flexibility without requiring the use of HTML or "`flavors`" for essential syntax such as tables, description lists, admonitions (tips, notes, warnings, etc.) and table of contents.

It's important to understand that AsciiDoc was initially designed as a plain-text alternative to the DocBook XML schema.
AsciiDoc isn't stuck in a game of whack-a-mole trying to satisfy publishing needs like Markdown.
Rather, the AsciiDoc syntax was explicitly designed with the needs of publishing in mind, both print and web.
If the need arises, you can make full use of the huge choice of tools available for a DocBook workflow using Asciidoctor's DocBook converter.
That's why mapping to an enterprise documentation format like DocBook remains a key use case for AsciiDoc.

And yet, AsciiDoc is simple enough to stand in as a better flavor of Markdown.
But what truly makes AsciiDoc the right investment is that its syntax was designed to be extended as a core feature.
This extensibility not only means that AsciiDoc has a lot more to offer, with room to grow, it also fulfills the objective of ensuring your content is maximally reusable.

== Comparison by example

The following table shows the AsciiDoc syntax as it compares to Markdown.
Since AsciiDoc supports a broader range of syntax than Markdown, this side-by-side comparison focuses mainly on areas where the syntax overlaps.

.A selection of AsciiDoc language features compared to Markdown
[#asciidoc-vs-markdown%autowidth]
|===
|Language Feature |Markdown |AsciiDoc

|Bold (constrained)
a|
[source,markdown]
----
**bold**
----
a|
[source]
----
*bold*
----

|Bold (unconstrained)
a|
[source,markdown]
----
**b**old
----
a|
[source]
----
**b**old
----

|Italic (constrained)
a|
[source,markdown]
----
*italic*
----
a|
[source]
----
_italic_
----

|Italic (unconstrained)
|_n/a_
a|
[source]
----
__i__talic
----

|Monospace (constrained)
a|
[source,markdown]
----
`monospace`
----
a|
[source]
----
`monospace`
----

|Monospace (unconstrained)
a|
[source,markdown]
----
`m`onospace
----
a|
[source]
----
``m``onospace
----

|Literal monospace
a|
[source,markdown]
----
`http://localhost:8080`
`/issue/{id}`
----
a|
[source]
----
`+http://localhost:8080+`
`+/issue/{id}+`
----

|Link with label
a|
[source,markdown]
----
[Asciidoctor](https://asciidoctor.org)
----
a|
[source]
----
https://asciidoctor.org[Asciidoctor]
----

|Relative link
a|
[source,markdown]
----
[user guide](user-guide.html)
----
a|
[source]
----
link:user-guide.html[user guide]
xref:user-guide.adoc[user guide]
----

|File link
a|
[source,markdown]
----
[get the PDF]({% raw %}{{ site.url }}{% endraw %}/assets/mydoc.pdf)
----
a|
[source]
----
link:{site-url}/assets/mydoc.pdf[get the PDF]
----

|Cross reference
a|
[source,markdown]
----
See [Usage](#_usage).

<h2 id="_usage">Usage</h2>
----
a|
[source]
----
See <<_usage>>.

== Usage
----

|Block ID (aka anchor)
a|
[source,markdown]
----
<h2 id="usage">Usage</h2>
----
a|
[source]
----
[#usage]
== Usage
----

|Inline anchor
|_n/a_
a|
[source]
----
. [[step-1]]Download the software
----

|Inline image w/ alt text
a|
[source,markdown]
----
![Logo](/images/logo.png)
----
a|
[source]
----
image:logo.png[Logo]
----

|Block image w/ alt text
|_n/a_
a|
[source]
----
image::logo.png[Logo]
----

|Section heading*
a|
[source,markdown]
----
## Heading 2
----
a|
[source]
----
== Heading 2
----

|Blockquote*
a|
[source,markdown]
----
> Quoted text.
>
> Another paragraph in quote.
----
a|
[source]
----
____
Quoted text.

Another paragraph in quote.
____
----

|Literal block
a|
[source,markdown]
----
    $ gem install asciidoctor
----
a|
.Indented (by 1 or more spaces)
[source]
----
 $ gem install asciidoctor
----

.Delimited
[source]
----
....
$ gem install asciidoctor
....
----

|Code block*
a|
[source,markdown]
----
```java
public class Person {
  private String name;
  public Person(String name) {
    this.name = name;
  }
}
```
----
a|
[source]
....
[source,java]
----
public class Person {
  private String name;
  public Person(String name) {
    this.name = name;
  }
}
----
....

|Unordered list
a|
[source,markdown]
----
* apples
* orange
  * temple
  * navel
* bananas
----
a|
[source]
----
* apples
* oranges
** temple
** navel
* bananas
----
|Ordered list
a|
[source,markdown]
----
1. first
2. second
3. third
----
a|
[source]
----
. first
. second
. third
----

|Thematic break (aka horizontal rule)*
a|
[source,markdown]
----
***

* * *

---

- - -

___

_ _ _
----
a|
[source]
----
'''
----

|Typographic quotes (aka "`smart quotes`")
|Enabled through an extension switch, but offer little control in how they are applied.
a|
[source]
----
The `'90s popularized a new form of music known as "`grunge`" rock.
It'll turn out to have an impact that extended well beyond music.
----

|Document header
a|
.Slapped on as "`front matter`"
[source,markdown]
----
---
layout: docs
title: Writing posts
prev_section: defining-frontmatter
next_section: creating-pages
permalink: /docs/writing-posts/
---
----
a|
.Native support!
[source]
----
= Writing posts
:page-layout: base
:showtitle:
:prev_section: defining-frontmatter
:next_section: creating-pages
----

|Admonitions
|_n/a_
a|
[source]
----
TIP: You can add line numbers to source listings by adding the word `numbered` in the attribute list after the language name.
----

|Sidebars
|_n/a_
a|
[source]
----
.Lightweight Markup
****
Writing languages that let you type less and express more.
****
----

|Block titles
|_n/a_
a|
[source]
----
.Grocery list
* Milk
* Eggs
* Bread
----

|Includes
|_n/a_
a|
[source]
----
\include::intro.adoc[]
----

|URI reference
a|
[source,markdown]
----
Go to the [home page][home].

[home]: https://example.org
----
a|
[source]
----
:home: https://example.org

Go to the {home}[home page].
----

|Custom CSS classes
|_n/a_
a|
[source]
----
[.path]_Gemfile_
----
|===

+*+ Asciidoctor also supports the Markdown syntax for this language feature.

You can see that AsciiDoc has the following advantages over Markdown:

* AsciiDoc uses the same number of markup characters or less when compared to Markdown in nearly all cases.
* AsciiDoc uses a consistent formatting scheme (i.e., it has consistent patterns).
* AsciiDoc can handle all permutations of nested inline (and block) formatting, whereas Markdown often falls down.
* AsciiDoc handles cases that Markdown doesn't, such as a proper approach to inner-word markup, source code blocks and block-level images.

NOTE: Certain Markdown flavors, such as Markdown Extra, support additional features such as tables and description lists.
However, since these features don't appear in "`plain`" Markdown, they're not included in the comparison table.
But they're supported natively by AsciiDoc.

Asciidoctor, which is used for converting AsciiDoc on GitHub and GitLab, emulates some of the common parts of the Markdown syntax, like headings, blockquotes and fenced code blocks, simplifying the migration from Markdown to AsciiDoc.
For details, see xref:syntax-quick-reference.adoc#markdown-compatibility[Markdown compatibility].

////
===== Description Lists in AsciiDoc

[source]
----
a term:: a description
another term:: another description
----

They can even hold code examples:

[source]
....
term with code example:: a description
+
[source,java]
----
public class Person {
}
----
....

===== Tables in AsciiDoc

An AsciiDoc table can be written as a series of lists which use a vertical bar as the list marker:

[source]
----
[cols=3]
|===
|a
|b
|c

|1
|2
|3
|===
----

Which appears as:

[cols=3]
|===
|a
|b
|c

|1
|2
|3
|===

Markdown Extra supports tables and description lists, too; but it's not Markdown.
Also, unlike Markdown Extra, AsciiDoc can apply formatting to cells.
////
"#
);

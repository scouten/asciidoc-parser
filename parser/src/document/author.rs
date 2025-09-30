use std::sync::LazyLock;

use regex::Regex;

use crate::{
    Parser, Span,
    content::{Content, SubstitutionGroup},
};

/// Represents a single author as (typically) described on the [author line].
///
/// The attributes `firstname`, `middlename`, `lastname`, and `authorinitials`
/// are automatically derived from the full value of the author string. When
/// assigned implicitly via the author line, the value includes all of the
/// characters and words prior to the semicolon (`;`), angle bracket (`<`), or
/// the end of the line. Note that when using the implicit author line, the full
/// name can have a maximum of three space-separated names. If it has more, then
/// the full name is assigned to the `firstname` attribute. You can adjoin names
/// using an underscore (`_`) character.
///
/// [author line]: https://docs.asciidoctor.org/asciidoc/latest/document/author-line/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Author {
    name: String,
    firstname: String,
    middlename: Option<String>,
    lastname: Option<String>,
    email: Option<String>,
}

impl Author {
    pub(crate) fn parse(source: &str, parser: &Parser) -> Option<Self> {
        let source = source.trim();
        if source.is_empty() {
            return None;
        }

        let (name_without_email, _) = source.split_once('<').unwrap_or((source, ""));

        let author_source = if name_without_email.contains('{') {
            apply_header_subs(source, parser)
        } else {
            source.to_string()
        };

        if let Some(captures) = AUTHOR.captures(&author_source) {
            let name = name_without_email.trim().to_string();
            let firstname = captures[1].to_string();
            let mut middlename = captures.get(2).map(|m| m.as_str().to_string());
            let mut lastname = captures.get(3).map(|m| m.as_str().to_string());
            let email = captures
                .get(4)
                .map(|m| apply_header_subs(m.as_str(), parser));

            if middlename.is_some() && lastname.is_none() {
                lastname = middlename;
                middlename = None;
            }

            Some(Self {
                name,
                firstname,
                middlename,
                lastname,
                email,
            })
        } else {
            // AsciiDoc syntax doesn't allow more than three space-separated
            // names to be parsed into first/middle/last/email. In that case,
            // we get simple and just treat it all as first name.

            let name = apply_header_subs(&author_source, parser);

            Some(Self {
                name: name.clone(),
                firstname: name,
                middlename: None,
                lastname: None,
                email: None,
            })
        }
    }

    /// Returns the full name of the author.
    ///
    /// The name includes the entire author declaration except for email.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the first, forename, or given name of the author.
    ///
    /// The first space-separated name in the value of the `author` attribute is
    /// automatically assigned to `firstname`.
    pub fn firstname(&self) -> &str {
        &self.firstname
    }

    /// Returns the middle name or initial of the author.
    ///
    /// If author contains three space-separated names, the second name is
    /// assigned to the `middlename` attribute.
    pub fn middlename(&self) -> Option<&str> {
        self.middlename.as_deref()
    }

    /// Returns the last, surname, or family name of the author.
    ///
    /// If the author name contains two or three space-separated names, the last
    /// of those names is assigned to the `lastname` attribute.
    pub fn lastname(&self) -> Option<&str> {
        self.lastname.as_deref()
    }

    /// Returns the email address or URL associated with the author.
    ///
    /// When assigned via the author line, it’s enclosed in a pair of angle
    /// brackets (`< >`). A URL can be used in place of the email address.
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    /// Returns the initials of the author.
    ///
    /// The first character of the `firstname`, `middlename`, and `lastname`
    /// attribute values are assigned to the `authorinitials` attribute. The
    /// value of the `authorinitials` attribute will consist of three characters
    /// or less depending on how many parts are in the author’s name.
    pub fn initials(&self) -> String {
        format!(
            "{first}{middle}{last}",
            first = first_char_or_empty_string(&self.firstname),
            middle = opt_first_char_or_empty_string(self.middlename.as_deref()),
            last = opt_first_char_or_empty_string(self.lastname.as_deref()),
        )
    }
}

fn first_char_or_empty_string(s: &str) -> String {
    s.chars().next().map_or(String::new(), |c| c.to_string())
}

fn opt_first_char_or_empty_string(s: Option<&str>) -> String {
    s.map(first_char_or_empty_string).unwrap_or_default()
}

static AUTHOR: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(
        r#"(?x)
            ^

            # Group 1: First name (required)
            ([a-zA-Z0-9_\p{L}\p{N}][a-zA-Z0-9_\p{L}\p{N}\-'.]*)

            # Group 2: Middle name (optional)
            (?:\ +([a-zA-Z0-9_\p{L}\p{N}][a-zA-Z0-9_\p{L}\p{N}\-'.]*))?

            # Group 3: Last name (optional)
            (?:\ +([a-zA-Z0-9_\p{L}\p{N}][a-zA-Z0-9_\p{L}\p{N}\-'.]*))?

            # Group 4: Email address (optional)
            (?:\ +<([^>]+)>)?

            $
        "#,
    )
    .unwrap()
});

fn apply_header_subs(source: &str, parser: &Parser) -> String {
    let span = Span::new(source);

    let mut content = Content::from(span);
    SubstitutionGroup::Header.apply(&mut content, parser, None);

    content.rendered().to_string()
}

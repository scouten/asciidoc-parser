use std::sync::LazyLock;

use regex::Regex;

use crate::{Parser, Span, content::Content};

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

        // Match Ruby's behavior: parse the raw input first to extract components,
        // then apply attribute substitution to individual components afterwards.
        // Special case: if the entire input is a single attribute reference,
        // treat the expanded result as a single name.

        let is_single_attribute = source.trim().starts_with('{')
            && source.trim().ends_with('}')
            && source.matches('{').count() == 1;

        if is_single_attribute {
            // Entire input is a single attribute reference - expand and treat as single name
            // to match Ruby Asciidoctor behavior
            let expanded_source = apply_author_subs(source, parser);

            Some(Self {
                name: expanded_source.clone(),
                firstname: expanded_source,
                middlename: None,
                lastname: None,
                email: None,
            })
        } else if let Some(captures) = AUTHOR.captures(source) {
            // Raw input matches author pattern - extract components then apply substitutions
            let name_without_email = source.split_once('<').unwrap_or((source, "")).0.trim();
            let name = name_without_email.to_string();

            // Extract raw components first
            let firstname = apply_author_subs(&captures[1], parser);
            let mut middlename = captures
                .get(2)
                .map(|m| apply_author_subs(m.as_str(), parser));
            let mut lastname = captures
                .get(3)
                .map(|m| apply_author_subs(m.as_str(), parser));
            let email = captures
                .get(4)
                .map(|m| apply_author_subs(m.as_str(), parser));

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
        } else if source.contains('{') {
            // Input contains attributes that prevent regex match - expand first, then try parsing
            let expanded_source = apply_author_subs(source, parser);
            
            if let Some(captures) = AUTHOR.captures(&expanded_source) {
                // After expansion, it matches the pattern - parse normally
                let name_without_email = expanded_source.split_once('<').unwrap_or((&expanded_source, "")).0.trim();
                let name = name_without_email.to_string();

                let firstname = captures[1].to_string();
                let mut middlename = captures.get(2).map(|m| m.as_str().to_string());
                let mut lastname = captures.get(3).map(|m| m.as_str().to_string());
                let email = captures.get(4).map(|m| m.as_str().to_string());

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
                // Even after expansion, doesn't match - treat as single name with HTML encoding
                let mut expanded_name = expanded_source;
                
                if expanded_name.contains('<') && expanded_name.contains('>') {
                    let span = crate::Span::new(&expanded_name);
                    let mut content = crate::content::Content::from(span);
                    crate::content::SubstitutionStep::SpecialCharacters.apply(&mut content, parser, None);
                    expanded_name = content.rendered().to_string();
                }

                Some(Self {
                    name: expanded_name.clone(),
                    firstname: expanded_name,
                    middlename: None,
                    lastname: None,
                    email: None,
                })
            }
        } else {
            // Input doesn't contain attributes and doesn't match pattern - treat as single name
            let mut name = source.to_string();
            
            // Apply HTML encoding for unparseable patterns that contain angle brackets
            if name.contains('<') && name.contains('>') {
                let span = crate::Span::new(&name);
                let mut content = crate::content::Content::from(span);
                crate::content::SubstitutionStep::SpecialCharacters.apply(&mut content, parser, None);
                name = content.rendered().to_string();
            }
            
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

            # Group 1: First name (required) - allow HTML entities
            ([a-zA-Z0-9_\p{L}\p{N}&\#;][a-zA-Z0-9_\p{L}\p{N}\-'.&\#;]*)

            # Group 2: Middle name (optional) - allow HTML entities
            (?:\ +([a-zA-Z0-9_\p{L}\p{N}&\#;][a-zA-Z0-9_\p{L}\p{N}\-'.&\#;]*))?

            # Group 3: Last name (optional) - allow HTML entities
            (?:\ +([a-zA-Z0-9_\p{L}\p{N}&\#;][a-zA-Z0-9_\p{L}\p{N}\-'.&\#;]*))?

            # Group 4: Email address (optional)
            (?:\ +<([^>]+)>)?

            $
        "#,
    )
    .unwrap()
});

// Apply header substitutions to individual author components.
// This matches Ruby's Document#apply_header_subs method behavior.
pub(crate) fn apply_author_subs(source: &str, parser: &Parser) -> String {
    let span = Span::new(source);
    let mut content = Content::from(span);

    use crate::content::SubstitutionStep;

    // Apply attribute references first
    SubstitutionStep::AttributeReferences.apply(&mut content, parser, None);

    // Apply HTML encoding based on Ruby's behavior:
    // - Single attribute reference (like {full-author}): No HTML encoding
    // - Single attribute in email position (like <{email}>): No HTML encoding  
    // - Multiple attributes or complex patterns: HTML encoding
    // - Don't HTML encode if the content only has pre-existing HTML entities
    let is_simple_single_attribute = source.trim().starts_with('{')
        && source.trim().ends_with('}')
        && source.matches('{').count() == 1;
    
    let has_multiple_attributes = source.matches('{').count() > 1;
    
    // Check if we should apply HTML encoding
    let rendered = content.rendered();
    let has_angle_brackets = rendered.contains('<') && rendered.contains('>');
    let has_unencoded_ampersand = rendered.contains('&') && !rendered.contains("&amp;");
    
    if !is_simple_single_attribute 
        && has_multiple_attributes
        && (has_angle_brackets || has_unencoded_ampersand)
    {
        SubstitutionStep::SpecialCharacters.apply(&mut content, parser, None);
    }

    content.rendered().to_string()
}

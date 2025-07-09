use std::sync::LazyLock;

use regex::Regex;

/// A `PathResolver` handles all operations for resolving, cleaning, and joining
/// paths. This struct includes operations for handling both web paths (request
/// URIs) and system paths.
///
/// The main emphasis of the struct is on creating clean and secure paths. Clean
/// paths are void of duplicate parent and current directory references in the
/// path name. Secure paths are paths which are restricted from accessing
/// directories outside of a jail path, if specified.
///
/// Since joining two paths can result in an insecure path, this struct also
/// handles the task of joining a parent (start) and child (target) path.
///
/// Like its counterpart in the Ruby Asciidoctor implementation, this struct
/// makes no use of path utilities from the underlying Rust libraries. Instead,
/// it handles all aspects of path manipulation. The main benefit of
/// internalizing these operations is that the struct is able to handle both
/// Posix and Windows paths independent of the operating system on which it
/// runs. This makes the class both deterministic and easier to test.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathResolver {
    /// File separator to use for path operations. (Defaults to
    /// platform-appropriate separator.)
    pub file_separator: char,
    // TO DO: Port this from Ruby?
    // attr_accessor :working_dir
}

impl Default for PathResolver {
    fn default() -> Self {
        Self {
            file_separator: std::path::MAIN_SEPARATOR,
        }
    }
}

impl PathResolver {
    /// Normalize path by converting any backslashes to forward slashes.
    pub fn posixify(&self, path: &str) -> String {
        if self.file_separator == '\\' && path.contains('\\') {
            path.replace('\\', "/")
        } else {
            path.to_string()
        }
    }

    /// Resolve a web path from the target and start paths.
    ///
    /// The main function of this operation is to resolve any parent references
    /// and remove any self references.
    ///
    /// The target is assumed to be a path, not a qualified URI. That check
    /// should happen before this method is invoked.
    ///
    /// Returns a path that joins the target path with the start path with any
    /// parent references resolved and self references removed.
    #[allow(unused)] // TEMPORARY while building
    pub fn web_path(&self, target: &str, start: Option<&str>) -> String {
        let mut target = self.posixify(target);
        let start = start.map(|start| self.posixify(start));

        dbg!(&target);
        dbg!(&start);

        let mut uri_prefix: Option<String> = None;

        if start.is_some() || self.is_web_root(&target) {
            (target, uri_prefix) = extract_uri_prefix(&format!(
                "{start}{maybe_add_slash}{target}",
                start = start.as_deref().unwrap_or_default(),
                maybe_add_slash = start
                    .as_ref()
                    .map(|s| if s.ends_with("/") { "" } else { "/" })
                    .unwrap_or_default()
            ));
        }

        dbg!(&target);
        dbg!(&uri_prefix);

        let (target_segments, target_root) = self.partition_path(&target, WebPath(true));

        dbg!(&target_segments);
        dbg!(&target_root);

        let mut resolved_segments: Vec<String> = vec![];

        for segment in target_segments {
            if segment == ".." {
                if resolved_segments.is_empty() {
                    if let Some(target_root) = target_root.as_ref()
                        && target_root != ".'"
                    {
                        // Do nothing.
                    } else {
                        resolved_segments.push(segment);
                    }
                } else if let Some(last_segment) = resolved_segments.last()
                    && last_segment == ".."
                {
                    resolved_segments.push(segment);
                } else {
                    resolved_segments.pop();
                }
            } else {
                resolved_segments.push(segment);
            }
        }

        let resolved_path = self
            .join_path(&resolved_segments, target_root.as_deref())
            .replace(" ", "%20");

        format!(
            "{uri_prefix}{resolved_path}",
            uri_prefix = uri_prefix.unwrap_or_default()
        )
    }

    /// Partition the path into path segments and remove self references (`.`)
    /// and the trailing slash, if present. Prior to being partitioned, the path
    /// is converted to a Posix path.
    ///
    /// Parent references are not resolved by this method since the caller often
    /// needs to handle this resolution in a certain context (checking for the
    /// breach of a jail, for instance).
    ///
    /// Returns a 2-item tuple containing a `Vec<String>` of path segments and
    /// an optional path root (e.g., `/`, `./`, `c:/`, or `//`), which is only
    /// present if the path is absolute.
    fn partition_path(&self, path: &str, web: WebPath) -> (Vec<String>, Option<String>) {
        // TO DO: Add cache implementation?

        let posix_path = self.posixify(path);

        let root: Option<String> = if web.0 {
            if self.is_web_root(&posix_path) {
                Some("/".to_owned())
            } else if posix_path.starts_with("./") {
                Some("./".to_owned())
            } else {
                None
            }
        } else {
            todo!(
                "Port this: {}",
                r#"
				elsif root? posix_path
				  # ex. //sample/path
				  if unc? posix_path
					root = DOUBLE_SLASH
				  # ex. /sample/path
				  elsif posix_path.start_with? SLASH
					root = SLASH
				  # ex. uri:classloader:sample/path (or uri:classloader:/sample/path)
				  elsif posix_path.start_with? URI_CLASSLOADER
					root = posix_path.slice 0, URI_CLASSLOADER.length
				  # ex. C:/sample/path (or file:///sample/path in browser environment)
				  else
					root = posix_path.slice 0, (posix_path.index SLASH) + 1
				  end
				# ex. ./sample/path
				elsif posix_path.start_with? DOT_SLASH
				  root = DOT_SLASH
				end
				# otherwise ex. sample/path
                "#
            );
        };

        let path_after_root = if let Some(root) = &root {
            &posix_path[root.len()..]
        } else {
            &posix_path
        };

        let path_segments: Vec<String> = path_after_root
            .split('/')
            .filter(|s| *s != ".")
            .map(|s| s.to_owned())
            .collect();

        // TO DO: Add cache write?

        (path_segments, root)
    }

    /// Join the segments using the Posix file separator (since this crate knows
    /// how to work with paths specified this way, regardless of OS). Use the
    /// `root`, if specified, to construct an absolute path. Otherwise join the
    /// segments as a relative path.
    fn join_path(&self, segments: &[String], root: Option<&str>) -> String {
        format!(
            "{root}{segments}",
            root = root.unwrap_or_default(),
            segments = segments.join("/"),
        )
    }

    /// Return `true` if the path is an absolute (root) web path (i.e. starts
    /// with a `'/'`.
    pub fn is_web_root(&self, path: &str) -> bool {
        path.starts_with('/')
    }
}

/// Efficiently extracts the URI prefix from the specified string if the string
/// is a URI.
///
/// Attempts to match the URI prefix in the specified string (e.g., `http://`). If present, the prefix is removed.
///
/// Returns a tuple containing the specified string without the URI prefix, if
/// present, and the extracted URI prefix if found.
fn extract_uri_prefix(s: &str) -> (String, Option<String>) {
    if s.contains(':') {
        if let Some(prefix) = URI_SNIFF.find(s) {
            return (
                s[prefix.len()..].to_string(),
                Some(prefix.as_str().to_owned()),
            );
        }
    }

    (s.to_string(), None)
}

static URI_SNIFF: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(
        r#"(?x)
        ^                   # Anchor: start of string

        \p{Alphabetic}      # First character: a Unicode letter

        [\p{Alphabetic}     # Followed by one or more of:
         \p{Number}         #   - Unicode letters or numbers
         .                  #   - Period
         \+                 #   - Plus sign
         \-                 #   - Hyphen
        ]+                  # One or more of the above

        :                   # Followed by a literal colon

        /{0,2}              # Followed by zero, one, or two literal slashes
    "#,
    )
    .unwrap()
});

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WebPath(pub(crate) bool);

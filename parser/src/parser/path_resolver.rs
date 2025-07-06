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
    pub fn web_path(&self, _target: &str, _start: Option<&str>) -> String {
        todo!(
            "Port this: {}",
            r#"
			target = posixify target
			start = posixify start
		
			unless start.nil_or_empty? || (web_root? target)
			  target, uri_prefix = extract_uri_prefix %(#{start}#{(start.end_with? SLASH) ? '' : SLASH}#{target})
			end
		
			# use this logic instead if we want to normalize target if it contains a URI
			#unless web_root? target
			#  target, uri_prefix = extract_uri_prefix target if preserve_uri_target
			#  target, uri_prefix = extract_uri_prefix %(#{start}#{SLASH}#{target}) unless uri_prefix || start.nil_or_empty?
			#end
		
			target_segments, target_root = partition_path target, true
			resolved_segments = []
			target_segments.each do |segment|
			  if segment == DOT_DOT
				if resolved_segments.empty?
				  resolved_segments << segment unless target_root && target_root != DOT_SLASH
				elsif resolved_segments[-1] == DOT_DOT
				  resolved_segments << segment
				else
				  resolved_segments.pop
				end
			  else
				resolved_segments << segment
				# checking for empty would eliminate repeating forward slashes
				#resolved_segments << segment unless segment.empty?
			  end
			end
		
			if (resolved_path = join_path resolved_segments, target_root).include? ' '
			  resolved_path = resolved_path.gsub ' ', '%20'
			end
		
			uri_prefix ? %(#{uri_prefix}#{resolved_path}) : resolved_path
        "#
        );
    }
}

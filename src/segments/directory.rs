use std::path::{Path, PathBuf};

use crate::configs::directory::ConfigAlias;
use crate::context::Context;
use crate::segments::Segment;

fn shrink_file_name(file_name: &str, shrink_len: usize) -> String {
    let n = if file_name.starts_with('.') {
        shrink_len + 1
    } else {
        shrink_len
    };
    file_name.chars().take(n).collect()
}

#[test]
fn test_shrink_file_name() {
    assert_eq!(shrink_file_name("abc", 1), "a");
    assert_eq!(shrink_file_name("abc", 2), "ab");
    assert_eq!(shrink_file_name("abc", 5), "abc");

    assert_eq!(shrink_file_name(".abc", 1), ".a");
    assert_eq!(shrink_file_name(".abc", 2), ".ab");
    assert_eq!(shrink_file_name(".abc", 5), ".abc");
}

const PATH_SEPARATOR: &str = "/";

fn shrink_path(
    shrink_enabled: bool,
    shrink_len: usize,
    cwd: &Path,
    aliases: &[PathAlias],
    repo_dir: Option<&Path>,
) -> String {
    // "~/abc/def" -> ["def", "abc", "~"]
    let mut reversed_path_segments: Vec<String> = vec![];

    let mut dir = cwd;

    'outer: loop {
        let first = reversed_path_segments.is_empty();

        for a in aliases {
            if dir == a.path {
                reversed_path_segments.push(a.alias.to_string());
                break 'outer;
            }
        }

        let is_repo_dir = Some(dir) == repo_dir;

        let file_name = dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let should_shrink = !first && !is_repo_dir && shrink_enabled;
        let shrinked_file_name = if should_shrink {
            shrink_file_name(&file_name, shrink_len)
        } else {
            file_name
        };

        reversed_path_segments.push(shrinked_file_name);

        dir = match dir.parent() {
            Some(parent) => parent,
            None => break,
        };
    }

    let mut path_segments = reversed_path_segments;
    path_segments.reverse();
    path_segments.join(PATH_SEPARATOR)
}

#[test]
fn test_shrink_path() {
    struct Scenario<'a> {
        test_name: &'a str,
        shrink_enabled: bool,
        shrink_len: usize,
        cwd: &'a str,
        aliases: &'a [PathAlias<'a>],
        repo_dir: Option<&'a str>,
        expected: &'a str,
    }

    let scenarios = &[
        Scenario {
            test_name: "at home, without shrinking",
            shrink_enabled: false,
            shrink_len: 1,
            cwd: "/home/test",
            aliases: &[PathAlias {
                path: PathBuf::from("/home/test"),
                alias: "~",
            }],
            repo_dir: None,
            expected: "~",
        },
        Scenario {
            test_name: "inside of home, without shrinking",
            shrink_enabled: false,
            shrink_len: 1,
            cwd: "/home/test/abc/.def",
            aliases: &[PathAlias {
                path: PathBuf::from("/home/test"),
                alias: "~",
            }],
            repo_dir: None,
            expected: "~/abc/.def",
        },
        Scenario {
            test_name: "outside of home, without shrinking",
            shrink_enabled: false,
            shrink_len: 1,
            cwd: "/abc/.def",
            aliases: &[PathAlias {
                path: PathBuf::from("/home/test"),
                alias: "~",
            }],
            repo_dir: None,
            expected: "/abc/.def",
        },
        Scenario {
            test_name: "at home, with shrinking",
            shrink_enabled: true,
            shrink_len: 1,
            cwd: "/home/test",
            aliases: &[PathAlias {
                path: PathBuf::from("/home/test"),
                alias: "~",
            }],
            repo_dir: None,
            expected: "~",
        },
        Scenario {
            test_name: "inside of home, with shrinking (len = 1)",
            shrink_enabled: true,
            shrink_len: 1,
            cwd: "/home/test/abc/.def",
            aliases: &[PathAlias {
                path: PathBuf::from("/home/test"),
                alias: "~",
            }],
            repo_dir: None,
            expected: "~/a/.def",
        },
        Scenario {
            test_name: "outside of home, with shrinking (len = 1)",
            shrink_enabled: true,
            shrink_len: 1,
            cwd: "/abc/.def",
            aliases: &[PathAlias {
                path: PathBuf::from("/home/test"),
                alias: "~",
            }],
            repo_dir: None,
            expected: "/a/.def",
        },
        Scenario {
            test_name: "inside of home, with shrinking (len = 2)",
            shrink_enabled: true,
            shrink_len: 2,
            cwd: "/home/test/abc/.def/g/h",
            aliases: &[PathAlias {
                path: PathBuf::from("/home/test"),
                alias: "~",
            }],
            repo_dir: None,
            expected: "~/ab/.de/g/h",
        },
        Scenario {
            test_name: "outside of home, with shrinking (len = 2)",
            shrink_enabled: true,
            shrink_len: 2,
            cwd: "/abc/.def/g/h",
            aliases: &[PathAlias {
                path: PathBuf::from("/home/test"),
                alias: "~",
            }],
            repo_dir: None,
            expected: "/ab/.de/g/h",
        },
        Scenario {
            test_name: "inside of home, inside of git repo",
            shrink_enabled: true,
            shrink_len: 1,
            cwd: "/home/test/repos/repo/ab/.cd/ef",
            aliases: &[PathAlias {
                path: PathBuf::from("/home/test"),
                alias: "~",
            }],
            repo_dir: Some("/home/test/repos/repo"),
            expected: "~/r/repo/a/.c/ef",
        },
        Scenario {
            test_name: "outside of home, inside of git repo",
            shrink_enabled: true,
            shrink_len: 1,
            cwd: "/repos/repo/ab/.cd/ef",
            aliases: &[PathAlias {
                path: PathBuf::from("/home/test"),
                alias: "~",
            }],
            repo_dir: Some("/repos/repo"),
            expected: "/r/repo/a/.c/ef",
        },
        Scenario {
            test_name: "at home, at git repo",
            shrink_enabled: true,
            shrink_len: 1,
            cwd: "/home/test",
            aliases: &[PathAlias {
                path: PathBuf::from("/home/test"),
                alias: "~",
            }],
            repo_dir: Some("/home/test"),
            expected: "~",
        },
        Scenario {
            test_name: "alias",
            shrink_enabled: true,
            shrink_len: 1,
            cwd: "/home/test/repos/abc",
            aliases: &[
                PathAlias {
                    path: PathBuf::from("/home/test/repos"),
                    alias: "@git",
                },
                PathAlias {
                    path: PathBuf::from("/home/test"),
                    alias: "~",
                },
            ],
            repo_dir: Some("/home/test/repos/abc"),
            expected: "@git/abc",
        },
    ];

    for s in scenarios {
        let actual = shrink_path(
            s.shrink_enabled,
            s.shrink_len,
            Path::new(s.cwd),
            s.aliases,
            s.repo_dir.map(Path::new),
        );

        assert_eq!(actual, s.expected, "{}", s.test_name);
    }
}

fn expand_tilde(path: &str, home_dir: &Option<PathBuf>) -> PathBuf {
    let home_dir = match home_dir {
        Some(home) => home,
        None => return PathBuf::from(path),
    };

    if path == "~" {
        home_dir.clone()
    } else if let Some(rest) = path.strip_prefix("~/") {
        let mut result = home_dir.clone();
        result.push(rest);
        result
    } else {
        PathBuf::from(path)
    }
}

#[test]
fn test_expand_tilde() {
    let home = Some(PathBuf::from("/home/test"));

    assert_eq!(expand_tilde("~", &home), PathBuf::from("/home/test"));
    assert_eq!(expand_tilde("~/", &home), PathBuf::from("/home/test"));
    assert_eq!(
        expand_tilde("/root/test", &home),
        PathBuf::from("/root/test")
    );
    assert_eq!(
        expand_tilde("~/abc", &home),
        PathBuf::from("/home/test/abc")
    );
}

#[derive(Debug)]
pub struct PathAlias<'a> {
    path: PathBuf,
    alias: &'a str,
}

fn path_aliases<'a>(
    aliases: &'a [ConfigAlias],
    home_dir: Option<PathBuf>,
    home_symbol: &'a str,
) -> Vec<PathAlias<'a>> {
    let mut result = Vec::new();
    result.reserve(aliases.len() + 1);

    for a in aliases {
        result.push(PathAlias {
            path: expand_tilde(&a.path, &home_dir),
            alias: &a.alias,
        });
    }

    if let Some(home_dir) = home_dir {
        result.push(PathAlias {
            path: home_dir,
            alias: home_symbol,
        });
    }

    result
}

pub fn build_segment<'ctx>(context: &'ctx Context) -> Option<Segment<'ctx>> {
    let config = &context.config.directory;

    let cwd = context.current_dir.as_path();
    let home_dir = dirs::home_dir();
    let repo_dir = context.git_repo.as_ref().and_then(|repo| repo.workdir());

    let aliases = path_aliases(&config.aliases, home_dir, &config.home);

    let content = shrink_path(
        config.shrink.enabled,
        config.shrink.max_len,
        cwd,
        &aliases,
        repo_dir,
    );

    let style = if cwd.is_dir() {
        &config.normal.style
    } else {
        &config.error.style
    };

    Some(Segment { style, content })
}

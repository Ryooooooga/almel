use std::path::{Path, PathBuf};

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
    home_symbol: &str,
    shrink_enabled: bool,
    shrink_len: usize,
    cwd: &Path,
    home_dir: &Option<PathBuf>,
    _repo_dir: Option<&Path>,
) -> String {
    // "~/abc/def" -> ["def", "abc", "~"]
    let mut reversed_path_segments: Vec<String> = vec![];

    let mut dir = cwd;

    loop {
        let first = reversed_path_segments.is_empty();

        if Some(dir) == home_dir.as_deref() {
            reversed_path_segments.push(home_symbol.to_string());
            break;
        }

        let file_name = dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let shrinked_file_name = if !first && shrink_enabled {
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
    struct Scenario {
        test_name: &'static str,
        home_symbol: &'static str,
        shrink_enabled: bool,
        shrink_len: usize,
        cwd: &'static str,
        home_dir: &'static str,
        repo_dir: Option<&'static str>,
        expected: &'static str,
    }

    let scenarios = &[
        Scenario {
            test_name: "at home, without shrinking",
            home_symbol: "~",
            shrink_enabled: false,
            shrink_len: 1,
            cwd: "/home/test",
            home_dir: "/home/test",
            repo_dir: None,
            expected: "~",
        },
        Scenario {
            test_name: "inside of home, without shrinking",
            home_symbol: "~",
            shrink_enabled: false,
            shrink_len: 1,
            cwd: "/home/test/abc/.def",
            home_dir: "/home/test",
            repo_dir: None,
            expected: "~/abc/.def",
        },
        Scenario {
            test_name: "outside of home, without shrinking",
            home_symbol: "~",
            shrink_enabled: false,
            shrink_len: 1,
            cwd: "/abc/.def",
            home_dir: "/home/test",
            repo_dir: None,
            expected: "/abc/.def",
        },
        Scenario {
            test_name: "at home, with shrinking",
            home_symbol: "~",
            shrink_enabled: true,
            shrink_len: 1,
            cwd: "/home/test",
            home_dir: "/home/test",
            repo_dir: None,
            expected: "~",
        },
        Scenario {
            test_name: "inside of home, with shrinking (len = 1)",
            home_symbol: "~",
            shrink_enabled: true,
            shrink_len: 1,
            cwd: "/home/test/abc/.def",
            home_dir: "/home/test",
            repo_dir: None,
            expected: "~/a/.def",
        },
        Scenario {
            test_name: "outside of home, with shrinking (len = 1)",
            home_symbol: "~",
            shrink_enabled: true,
            shrink_len: 1,
            cwd: "/abc/.def",
            home_dir: "/home/test",
            repo_dir: None,
            expected: "/a/.def",
        },
        Scenario {
            test_name: "inside of home, with shrinking (len = 2)",
            home_symbol: "~",
            shrink_enabled: true,
            shrink_len: 2,
            cwd: "/home/test/abc/.def/g/h",
            home_dir: "/home/test",
            repo_dir: None,
            expected: "~/ab/.de/g/h",
        },
        Scenario {
            test_name: "outside of home, with shrinking (len = 2)",
            home_symbol: "~",
            shrink_enabled: true,
            shrink_len: 2,
            cwd: "/abc/.def/g/h",
            home_dir: "/home/test",
            repo_dir: None,
            expected: "/ab/.de/g/h",
        },
    ];

    for s in scenarios {
        let actual = shrink_path(
            s.home_symbol,
            s.shrink_enabled,
            s.shrink_len,
            Path::new(s.cwd),
            &Some(PathBuf::from(s.home_dir)),
            s.repo_dir.map(Path::new),
        );

        assert_eq!(actual, s.expected, "{}", s.test_name);
    }
}

pub fn build_segment<'ctx>(context: &'ctx Context) -> Option<Segment<'ctx>> {
    let config = &context.config.directory;

    let cwd = context.current_dir.as_path();
    let home_dir = dirs::home_dir();
    let repo_dir = context.git_repo.as_ref().and_then(|repo| repo.workdir());

    let content = shrink_path(
        &config.home,
        config.shrink.enabled,
        config.shrink.max_len,
        cwd,
        &home_dir,
        repo_dir,
    );

    let style = if cwd.is_dir() {
        &config.normal.style
    } else {
        &config.error.style
    };

    Some(Segment { style, content })
}

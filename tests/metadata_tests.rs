use git2okf::core::validator::{validate_git_url, validate_repository_access};

#[test]
fn test_url_validation() {
    assert!(validate_git_url("https://github.com/a/b").is_ok());
    assert!(validate_git_url("git@github.com:a/b.git").is_ok());

    let err = validate_git_url("ftp://unsupported.com").unwrap_err();
    assert!(err.to_string().contains("Invalid or unsupported"));

    let err2 = validate_repository_access("").unwrap_err();
    assert!(err2.to_string().contains("cannot be empty"));
}

use tempfile::tempdir;

#[test]
fn test_tempdir_cleanup() {
    let path = {
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();
        assert!(path.exists());
        path
    };
    // The tempdir went out of scope, so it should be deleted
    assert!(!path.exists());
}

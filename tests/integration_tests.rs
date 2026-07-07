use git2okf::core::types::{Framework, FrameworkDetection, LanguageStat};
use git2okf::detector::framework::detect_framework;
use git2okf::detector::language::analyze_files_and_languages;
use std::fs::{self, File};
use tempfile::tempdir;

// To test this properly as a library, we need lib.rs.
// Currently Git2OKF is just a binary project (main.rs).
// For the tests to compile, we would typically extract a lib.rs or test modules internally.
// As a placeholder, we create the integration_tests setup here assuming lib.rs will be added.
// Note: To make this work, src/lib.rs would need to be created exposing core, git, detector, etc.

#[test]
fn test_laravel_detection() {
    let dir = tempdir().unwrap();
    let path = dir.path();

    // Create mock files
    File::create(path.join("composer.json")).unwrap();
    File::create(path.join("artisan")).unwrap();
    fs::create_dir(path.join("routes")).unwrap();

    File::create(path.join("index.php")).unwrap();
    File::create(path.join("app.php")).unwrap();
    File::create(path.join("script.js")).unwrap();

    // Test language
    let (languages, stats) = analyze_files_and_languages(path).unwrap();
    assert_eq!(stats.total_files, 5);
    assert_eq!(stats.source_files, 3); // 2 php + 1 js

    let framework_det = detect_framework(path, &languages).unwrap();
    assert_eq!(framework_det.framework, Framework::Laravel);
    assert!(framework_det.confidence >= 80);
}

#[test]
fn test_nextjs_detection() {
    let dir = tempdir().unwrap();
    let path = dir.path();

    File::create(path.join("package.json")).unwrap();
    File::create(path.join("next.config.js")).unwrap();
    fs::create_dir(path.join("pages")).unwrap();

    File::create(path.join("app.tsx")).unwrap();
    File::create(path.join("index.ts")).unwrap();

    let (languages, _) = analyze_files_and_languages(path).unwrap();
    let framework_det = detect_framework(path, &languages).unwrap();

    assert_eq!(framework_det.framework, Framework::NextJs);
}

#[test]
fn test_rust_detection() {
    let dir = tempdir().unwrap();
    let path = dir.path();

    File::create(path.join("Cargo.toml")).unwrap();
    fs::create_dir(path.join("src")).unwrap();
    File::create(path.join("src").join("main.rs")).unwrap();

    let (languages, _) = analyze_files_and_languages(path).unwrap();
    let framework_det = detect_framework(path, &languages).unwrap();

    assert_eq!(framework_det.framework, Framework::RustCargo);
}

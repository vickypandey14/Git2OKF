use git2okf::core::types::Framework;
use git2okf::detector::framework::FrameworkDetector;
use git2okf::detector::language::LanguageDetector;
use git2okf::detector::Detector;
use std::fs::{self, File};
use tempfile::tempdir;

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
    let lang_detector = LanguageDetector;
    let (languages, stats) = lang_detector.detect(path).unwrap();
    assert_eq!(stats.total_files, 5);
    assert_eq!(stats.source_files, 3); // 2 php + 1 js

    let fw_detector = FrameworkDetector::new(languages);
    let framework_det = fw_detector.detect(path).unwrap();
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

    let lang_detector = LanguageDetector;
    let (languages, _) = lang_detector.detect(path).unwrap();
    let fw_detector = FrameworkDetector::new(languages);
    let framework_det = fw_detector.detect(path).unwrap();

    assert_eq!(framework_det.framework, Framework::NextJs);
}

#[test]
fn test_rust_detection() {
    let dir = tempdir().unwrap();
    let path = dir.path();

    File::create(path.join("Cargo.toml")).unwrap();
    fs::create_dir(path.join("src")).unwrap();
    File::create(path.join("src").join("main.rs")).unwrap();

    let lang_detector = LanguageDetector;
    let (languages, _) = lang_detector.detect(path).unwrap();
    let fw_detector = FrameworkDetector::new(languages);
    let framework_det = fw_detector.detect(path).unwrap();

    assert_eq!(framework_det.framework, Framework::RustCargo);
}

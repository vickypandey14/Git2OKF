use git2okf::detector::language::LanguageDetector;
use git2okf::detector::Detector;
use tempfile::tempdir;
use std::fs::File;

#[test]
fn test_language_detection() {
    let dir = tempdir().unwrap();
    let path = dir.path();
    
    File::create(path.join("main.rs")).unwrap();
    File::create(path.join("lib.rs")).unwrap();
    File::create(path.join("utils.js")).unwrap();
    
    let detector = LanguageDetector;
    let (languages, stats) = detector.detect(path).unwrap();
    
    assert_eq!(stats.source_files, 3);
    assert_eq!(languages.len(), 2);
    
    let rust_stat = languages.iter().find(|l| l.name == "rust").unwrap();
    assert_eq!(rust_stat.percentage, 67); // 2/3 * 100
}

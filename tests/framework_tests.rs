use git2okf::core::types::{Framework, FrameworkDetection, LanguageStat};
use git2okf::detector::framework::FrameworkDetector;
use git2okf::detector::Detector;
use tempfile::tempdir;
use std::fs::{self, File};

#[test]
fn test_laravel_confidence_scoring() {
    let dir = tempdir().unwrap();
    let path = dir.path();
    
    File::create(path.join("composer.json")).unwrap();
    File::create(path.join("artisan")).unwrap();
    fs::create_dir(path.join("routes")).unwrap();
    
    let langs = vec![LanguageStat { name: "php".to_string(), percentage: 100 }];
    let detector = FrameworkDetector::new(langs);
    
    let result = detector.detect(path).unwrap();
    assert_eq!(result.framework, Framework::Laravel);
    assert_eq!(result.confidence, 95); // 30 + 50 + 15
}

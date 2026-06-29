use crate::core::errors::Git2OkfError;
use crate::core::types::{Framework, FrameworkDetection, LanguageStat};
use crate::detector::Detector;
use std::path::Path;
use tracing::debug;

pub struct FrameworkDetector {
    languages: Vec<LanguageStat>,
}

impl FrameworkDetector {
    pub fn new(languages: Vec<LanguageStat>) -> Self {
        Self { languages }
    }
}

impl Detector for FrameworkDetector {
    type Output = FrameworkDetection;

    fn detect(&self, path: &Path) -> Result<Self::Output, Git2OkfError> {
        debug!("Starting framework detection");
        let has_file = |name: &str| path.join(name).exists();
        let has_dir = |name: &str| path.join(name).is_dir();

        let mut best_framework = Framework::Unknown;
        let mut highest_confidence = 0u8;

        let mut update_best = |fw: Framework, conf: u8| {
            if conf > highest_confidence {
                highest_confidence = conf;
                best_framework = fw;
            }
        };

        for lang in &self.languages {
            match lang.name.as_str() {
                "php" => {
                    let mut conf = 0;
                    if has_file("composer.json") { conf += 30; }
                    if has_file("artisan") { conf += 50; }
                    if has_dir("app/Console") || has_dir("routes") { conf += 15; }
                    if conf > 0 { update_best(Framework::Laravel, conf); }
                }
                "javascript" | "typescript" => {
                    let mut next_conf = 0;
                    let mut react_conf = 0;
                    let mut vue_conf = 0;
                    let mut express_conf = 0;

                    if has_file("package.json") {
                        next_conf += 20;
                        react_conf += 20;
                        vue_conf += 20;
                        express_conf += 20;
                    }

                    if has_file("next.config.js") || has_file("next.config.mjs") || has_file("next.config.ts") {
                        next_conf += 70;
                    }

                    if has_file("vue.config.js") || has_file("vite.config.ts") {
                        vue_conf += 50;
                    }

                    if has_dir("src/components") || has_dir("pages") {
                        react_conf += 30;
                        next_conf += 10;
                    }

                    update_best(Framework::NextJs, next_conf);
                    update_best(Framework::React, react_conf);
                    update_best(Framework::Vue, vue_conf);
                    update_best(Framework::NodeExpress, express_conf);
                }
                "python" => {
                    let mut django_conf = 0;
                    let mut flask_conf = 0;

                    if has_file("requirements.txt") || has_file("pyproject.toml") {
                        django_conf += 20;
                        flask_conf += 20;
                    }
                    if has_file("manage.py") { django_conf += 70; }
                    if has_file("app.py") || has_file("main.py") { flask_conf += 30; }

                    update_best(Framework::Django, django_conf);
                    update_best(Framework::Flask, flask_conf);
                }
                "rust" => {
                    let mut rust_conf = 0;
                    if has_file("Cargo.toml") { rust_conf += 50; }
                    if has_dir("src") { rust_conf += 20; }
                    if has_file("Cargo.lock") { rust_conf += 25; }
                    update_best(Framework::RustCargo, rust_conf);
                }
                "java" => {
                    let mut spring_conf = 0;
                    if has_file("pom.xml") || has_file("build.gradle") { spring_conf += 40; }
                    if has_dir("src/main/java") { spring_conf += 20; }
                    if has_file("src/main/resources/application.properties") || has_file("src/main/resources/application.yml") {
                        spring_conf += 35;
                    }
                    update_best(Framework::SpringBoot, spring_conf);
                }
                _ => {}
            }
        }

        if highest_confidence > 100 {
            highest_confidence = 100;
        }

        debug!("Detected framework {:?} with confidence {}", best_framework, highest_confidence);

        Ok(FrameworkDetection {
            framework: best_framework,
            confidence: highest_confidence,
        })
    }
}

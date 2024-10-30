use super::NodeVisitor;
use crate::rc_dom::{Handle, NodeData};
use html5ever::{Attribute, QualName};
use log::{debug, info};
use path_clean::PathClean;
use sanitize_filename;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tendril::StrTendril;

/// A visitor that moves assets to a central directory and updates their references in HTML.
pub struct AssetMoverVisitor {
    /// The source directory containing HTML files
    base_dir: PathBuf,
    /// The directory where assets will be moved
    assets_dir: PathBuf,
    /// The current HTML file's path relative to base_dir
    current_file_path: PathBuf,
    /// Set of asset file extensions to process
    asset_extensions: HashSet<String>,
    /// Track moved assets to avoid duplicates
    moved_assets: HashSet<PathBuf>,
    /// The assets prefix path
    assets_prefix: String,
}

impl AssetMoverVisitor {
    pub fn new<P: AsRef<Path>>(base_dir: P, assets_dir: P, current_file_path: P) -> Self {
        let asset_extensions: HashSet<String> = vec![
            "jpg", "jpeg", "png", "gif", "svg", "css", "js", "woff", "woff2", "ttf", "eot", "mp4",
            "webm", "ogg", "mov", "avi", "mkv",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
            assets_dir: assets_dir.as_ref().to_path_buf(),
            current_file_path: current_file_path.as_ref().to_path_buf(),
            asset_extensions,
            moved_assets: HashSet::new(),
            assets_prefix: "/assets".to_string(),
        }
    }

    pub fn with_assets_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.assets_prefix = prefix.into();
        self
    }

    fn process_asset_path<P: AsRef<Path>>(&mut self, relative_path: P) -> Option<String> {
        let relative_path_str = relative_path.as_ref().to_string_lossy();

        // Skip absolute URLs and data URLs
        if relative_path_str.starts_with("http") || relative_path_str.starts_with("data:") {
            return None;
        }

        // Skip if not an asset we're interested in
        if let Some(ext) = relative_path.as_ref().extension() {
            if !self
                .asset_extensions
                .contains(&ext.to_string_lossy().to_lowercase())
            {
                return None;
            }
        } else {
            return None;
        }

        // Convert relative path to absolute path and normalize it
        let current_dir = self.current_file_path.parent().unwrap_or(Path::new(""));
        let absolute_path = self.base_dir.join(current_dir).join(&relative_path).clean();

        if !absolute_path.exists() {
            debug!("Asset not found: {:?}", absolute_path);
            return None;
        }

        // Generate new asset path with sanitized filename
        let stem = absolute_path.file_stem().unwrap().to_string_lossy();

        let sanitized_stem = sanitize_filename::sanitize(stem)
            .to_lowercase()
            .replace(' ', "_");

        let extension = absolute_path.extension().map_or_else(String::new, |ext| {
            format!(".{}", ext.to_string_lossy().to_lowercase())
        });

        let new_filename = format!(
            "{}-{}{}",
            sanitized_stem,
            uuid::Uuid::new_v4().to_string(),
            extension
        );
        let new_path = self.assets_dir.join(&new_filename);

        // Move the asset if we haven't already
        if !self.moved_assets.contains(&absolute_path) {
            if let Err(e) = fs::create_dir_all(&self.assets_dir) {
                debug!("Failed to create assets directory: {}", e);
                return None;
            }
            if let Err(e) = fs::copy(&absolute_path, &new_path) {
                debug!("Failed to copy asset: {}", e);
                return None;
            }
            self.moved_assets.insert(absolute_path);
        }

        // Return the new path relative to the site root
        Some(format!("{}/{}", self.assets_prefix, new_filename))
    }
}

impl NodeVisitor for AssetMoverVisitor {
    fn visit_element(
        &mut self,
        name: &QualName,
        attrs: &RefCell<Vec<Attribute>>,
        template_contents: &RefCell<Option<Handle>>,
        mathml_annotation_xml_integration_point: bool,
        handle: &Handle,
    ) -> (Option<Handle>, bool) {
        let mut attrs_ref = attrs.borrow_mut();

        // Process attributes that might contain asset paths
        for attr in attrs_ref.iter_mut() {
            match (name.local.as_ref(), attr.name.local.as_ref()) {
                // Images and Video posters
                ("img", "src") |
                ("video", "poster") |
                // Scripts
                ("script", "src") |
                // Stylesheets
                ("link", "href") |
                // Video sources and general media
                // ("source", "src") => {
                //     info!("Processing <source> element with src: {}", attr.value);
                //     if let Some(new_path) = self.process_asset_path(&attr.value) {
                //         info!("Updated <source> src from {} to {}", attr.value, new_path);
                //         attr.value = new_path.into();
                //     } else {
                //         info!("Skipped processing <source> src: {}", attr.value);
                //     }
                // }
                (_, "src") |
                (_, "href") => {
                    if let Some(new_path) = self.process_asset_path(attr.value.as_ref()) {
                        attr.value = new_path.into();
                    }
                }
                _ => {}
            }
        }

        (None, true)
    }
}

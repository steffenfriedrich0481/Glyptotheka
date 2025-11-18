use crate::utils::error::AppError;
use image::{imageops, Rgba, RgbaImage};
use std::path::PathBuf;
use tracing::{error, info, warn};

pub struct CompositePreviewService {
    cache_dir: PathBuf,
}

impl CompositePreviewService {
    pub fn new(cache_dir: PathBuf) -> Self {
        let preview_dir = cache_dir.join("previews");
        if let Err(e) = std::fs::create_dir_all(&preview_dir) {
            error!("Failed to create preview directory: {}", e);
        }
        Self { cache_dir }
    }

    /// Generate a composite preview from up to 4 images
    pub fn generate_preview(
        &self,
        project_id: i64,
        image_paths: &[String],
    ) -> Result<PathBuf, AppError> {
        if image_paths.is_empty() {
            return Err(AppError::ValidationError(
                "No images provided for preview generation".to_string(),
            ));
        }

        let count = image_paths.len().min(4);
        info!(
            "Generating composite preview for project {} with {} images",
            project_id, count
        );

        // Load and resize images
        let mut images = Vec::new();
        for (idx, path) in image_paths.iter().take(4).enumerate() {
            match self.load_and_resize_image(path, 400, 400) {
                Ok(img) => images.push(img),
                Err(e) => {
                    warn!("Failed to load image {}: {}", path, e);
                    // Continue with remaining images
                    if idx == 0 {
                        // If first image fails, we can't generate a preview
                        return Err(e);
                    }
                }
            }
        }

        if images.is_empty() {
            return Err(AppError::ValidationError(
                "No valid images loaded for preview".to_string(),
            ));
        }

        // Create composite based on image count
        let composite = match images.len() {
            1 => self.create_single_preview(&images[0]),
            2 => self.create_two_image_preview(&images[0], &images[1]),
            3 => self.create_three_image_preview(&images[0], &images[1], &images[2]),
            _ => self.create_four_image_preview(&images[0], &images[1], &images[2], &images[3]),
        };

        // Save to cache
        let preview_path = self
            .cache_dir
            .join("previews")
            .join(format!("project_{}_composite.png", project_id));

        composite
            .save(&preview_path)
            .map_err(|e| AppError::InternalServer(format!("Failed to save preview: {}", e)))?;

        info!("Composite preview saved to {:?}", preview_path);
        Ok(preview_path)
    }

    /// Load an image and resize it to fit within the specified dimensions
    fn load_and_resize_image(
        &self,
        path: &str,
        width: u32,
        height: u32,
    ) -> Result<RgbaImage, AppError> {
        let img = image::open(path).map_err(|e| {
            AppError::InternalServer(format!("Failed to load image {}: {}", path, e))
        })?;

        let resized = img.resize_exact(width, height, imageops::FilterType::Lanczos3);
        Ok(resized.to_rgba8())
    }

    /// Create a preview with a single image (800x800)
    fn create_single_preview(&self, img: &RgbaImage) -> RgbaImage {
        let mut canvas = RgbaImage::from_pixel(800, 800, Rgba([255, 255, 255, 255]));
        
        // Resize to 800x800 and place in center
        let resized = imageops::resize(img, 800, 800, imageops::FilterType::Lanczos3);
        imageops::replace(&mut canvas, &resized, 0, 0);
        
        canvas
    }

    /// Create a preview with two images side by side
    fn create_two_image_preview(&self, img1: &RgbaImage, img2: &RgbaImage) -> RgbaImage {
        let mut canvas = RgbaImage::from_pixel(800, 800, Rgba([255, 255, 255, 255]));

        // Place images side by side
        imageops::replace(&mut canvas, img1, 0, 200);
        imageops::replace(&mut canvas, img2, 400, 200);

        canvas
    }

    /// Create a preview with three images (2 on top, 1 on bottom)
    fn create_three_image_preview(
        &self,
        img1: &RgbaImage,
        img2: &RgbaImage,
        img3: &RgbaImage,
    ) -> RgbaImage {
        let mut canvas = RgbaImage::from_pixel(800, 800, Rgba([255, 255, 255, 255]));

        // Top row: 2 images
        imageops::replace(&mut canvas, img1, 0, 0);
        imageops::replace(&mut canvas, img2, 400, 0);

        // Bottom: 1 image centered and stretched
        let stretched = imageops::resize(img3, 800, 400, imageops::FilterType::Lanczos3);
        imageops::replace(&mut canvas, &stretched, 0, 400);

        canvas
    }

    /// Create a preview with four images in a 2x2 grid
    fn create_four_image_preview(
        &self,
        img1: &RgbaImage,
        img2: &RgbaImage,
        img3: &RgbaImage,
        img4: &RgbaImage,
    ) -> RgbaImage {
        let mut canvas = RgbaImage::from_pixel(800, 800, Rgba([255, 255, 255, 255]));

        // 2x2 grid
        imageops::replace(&mut canvas, img1, 0, 0);
        imageops::replace(&mut canvas, img2, 400, 0);
        imageops::replace(&mut canvas, img3, 0, 400);
        imageops::replace(&mut canvas, img4, 400, 400);

        canvas
    }

    /// Delete preview for a project
    pub fn delete_preview(&self, project_id: i64) -> Result<(), AppError> {
        let preview_path = self
            .cache_dir
            .join("previews")
            .join(format!("project_{}_composite.png", project_id));

        if preview_path.exists() {
            std::fs::remove_file(&preview_path).map_err(|e| {
                AppError::InternalServer(format!("Failed to delete preview: {}", e))
            })?;
            info!("Deleted preview for project {}", project_id);
        }

        Ok(())
    }
}

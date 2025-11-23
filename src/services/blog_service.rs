use std::fs;
use std::path::Path;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use pulldown_cmark::{Parser, Options, html};
use crate::models::blog::{BlogPost, BlogMetadata};

const BLOG_DIR: &str = "blogs";

/// Load all blog posts from the blogs directory
pub fn load_all_posts() -> Result<Vec<BlogPost>, Box<dyn std::error::Error>> {
    let mut posts = Vec::new();
    
    // Check if blogs directory exists
    if !Path::new(BLOG_DIR).exists() {
        return Ok(posts);
    }
    
    // Read all markdown files from the blogs directory
    let entries = fs::read_dir(BLOG_DIR)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        // Only process .md files
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Some(slug) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(post) = load_post_by_slug(slug) {
                    posts.push(post);
                }
            }
        }
    }
    
    // Sort posts by date (newest first)
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    
    Ok(posts)
}

/// Load a specific blog post by its slug (filename without extension)
pub fn load_post_by_slug(slug: &str) -> Result<BlogPost, Box<dyn std::error::Error>> {
    let file_path = format!("{}/{}.md", BLOG_DIR, slug);
    let content = fs::read_to_string(&file_path)?;
    
    // Parse frontmatter and content
    let matter = Matter::<YAML>::new();
    let result = matter.parse(&content);
    
    // Extract metadata
    let metadata: BlogMetadata = result
        .data
        .ok_or("No frontmatter found")?
        .deserialize()?;
    
    // Convert markdown to HTML
    let html_content = markdown_to_html(&result.content);
    
    Ok(BlogPost::new(slug.to_string(), metadata, html_content))
}

/// Convert markdown content to HTML
fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    
    let parser = Parser::new_ext(markdown, options);
    
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    html_output
}

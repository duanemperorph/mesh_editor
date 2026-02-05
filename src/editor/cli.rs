use clap::Parser;
use std::path::PathBuf;

use crate::mesh_document::{MeshDocument, MeshDocumentError};

/// Custom 3D mesh editor
#[derive(Parser)]
#[command(name = "mesh_editor")]
#[command(about = "A custom 3D mesh editor", long_about = None)]
pub struct Cli {
    /// Folder containing the mesh files (defaults to current directory)
    #[arg(default_value = ".")]
    mesh_folder: PathBuf,

    /// Open a specific version number (exits if not found)
    #[arg(short = 'v', long = "version")]
    version: Option<i32>,
}

impl Cli {
    pub fn load_document(&self) -> Result<MeshDocument, MeshDocumentError> {
        let folder_path = if self.mesh_folder.is_relative() {
            std::env::current_dir()
                .expect("Failed to get current directory")
                .join(&self.mesh_folder)
        } else {
            self.mesh_folder.clone()
        };

        // Ensure the folder exists, create it if needed (unless loading a specific version)
        if !folder_path.exists() {
            if self.version.is_some() {
                return Err(MeshDocumentError::DirectoryNotFound(folder_path));
            }
            std::fs::create_dir_all(&folder_path).expect("Failed to create mesh folder");
        }

        match self.version {
            Some(version) => MeshDocument::from_version(folder_path, version),
            None => MeshDocument::from_folder(folder_path),
        }
    }
}

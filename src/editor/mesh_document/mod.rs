//
// mesh_document -> wraps editing of a mesh
// Includes undo / redo as well as serialization
//

mod error;

pub use error::MeshDocumentError;

use mesh_editor::mesh::Mesh;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

const AUTO_SAVE_INTERVAL_SECS: u64 = 60;
const CURRENT_MESH_FILENAME: &str = "current.mesh";

pub struct MeshDocument {
    current_mesh: Mesh,
    last_saved_mesh: Option<Mesh>,
    folder_path: PathBuf,
    last_auto_save_check: Instant,
}

impl MeshDocument {
    pub fn new() -> MeshDocument {
        Self::with_mesh(Mesh::new())
    }

    pub fn with_mesh(mesh: Mesh) -> MeshDocument {
        MeshDocument {
            current_mesh: mesh,
            last_saved_mesh: None,
            folder_path: PathBuf::new(),
            last_auto_save_check: Instant::now(),
        }
    }

    pub fn from_folder(folder_path: PathBuf) -> Result<Self, MeshDocumentError> {
        if !folder_path.exists() {
            return Err(MeshDocumentError::DirectoryNotFound(folder_path));
        }

        let current_file = folder_path.join(CURRENT_MESH_FILENAME);
        let mesh = if current_file.exists() {
            Self::load_mesh_from_file(&current_file)?
        } else {
            Mesh::new()
        };

        Ok(MeshDocument {
            current_mesh: mesh.clone(),
            last_saved_mesh: Some(mesh),
            folder_path,
            last_auto_save_check: Instant::now(),
        })
    }

    /// Initialize from a specific version number
    pub fn from_version(folder_path: PathBuf, version: i32) -> Result<Self, MeshDocumentError> {
        if !folder_path.exists() {
            return Err(MeshDocumentError::DirectoryNotFound(folder_path.clone()));
        }

        let version_file = folder_path.join(format!("version_{}.mesh", version));
        if !version_file.exists() {
            return Err(MeshDocumentError::VersionNotFound(version));
        }

        let mesh = Self::load_mesh_from_file(&version_file)?;

        Ok(MeshDocument {
            current_mesh: mesh.clone(),
            last_saved_mesh: Some(mesh),
            folder_path,
            last_auto_save_check: Instant::now(),
        })
    }

    pub fn current_mesh(&self) -> &Mesh {
        &self.current_mesh
    }

    pub fn current_mesh_mut(&mut self) -> &mut Mesh {
        &mut self.current_mesh
    }
}

//
// MeshDocument -> Seralization Functions
//
impl MeshDocument {
    pub fn has_unsaved_changes(&self) -> bool {
        match &self.last_saved_mesh {
            Some(saved) => self.current_mesh != *saved,
            None => true,
        }
    }

    ///
    /// tick_auto_save -> should be called every frame
    ///
    pub fn tick_auto_save(&mut self) -> Result<(), MeshDocumentError> {
        if self.folder_path.as_os_str().is_empty() {
            return Ok(());
        }

        let elapsed = self.last_auto_save_check.elapsed();
        if elapsed.as_secs() >= AUTO_SAVE_INTERVAL_SECS {
            self.last_auto_save_check = Instant::now();

            if self.has_unsaved_changes() {
                self.save_current()?;
            }
        }
        Ok(())
    }

    ///
    /// Save to current.mesh (working copy version)
    ///
    pub fn save_current(&mut self) -> Result<(), MeshDocumentError> {
        if self.folder_path.as_os_str().is_empty() {
            return Ok(());
        }

        let path = self.folder_path.join(CURRENT_MESH_FILENAME);
        self.save_mesh_to_file(&self.current_mesh, &path)?;
        self.last_saved_mesh = Some(self.current_mesh.clone());
        Ok(())
    }

    ///
    /// Saves current.mesh and version_x.mesh
    ///
    pub fn save_version(&mut self) -> Result<i32, MeshDocumentError> {
        if self.folder_path.as_os_str().is_empty() {
            return Ok(0);
        }

        // Save current.mesh
        self.save_current()?;

        // Find next available version number
        let version = self.find_next_version_number()?;

        // Save versioned file
        let version_path = self.folder_path.join(format!("version_{}.mesh", version));
        self.save_mesh_to_file(&self.current_mesh, &version_path)?;

        Ok(version)
    }

    ///
    /// Restore to last saved version
    /// TODO: Fix me
    ///
    pub fn restore_to_last_saved(&mut self) -> bool {
        if let Some(ref saved) = self.last_saved_mesh {
            self.current_mesh = saved.clone();
            true
        } else {
            false
        }
    }

    ///
    /// Restore to a specific version
    ///
    pub fn restore_to_version(&mut self, version: i32) -> Result<(), MeshDocumentError> {
        let version_path = self.folder_path.join(format!("version_{}.mesh", version));
        if !version_path.exists() {
            return Err(MeshDocumentError::VersionNotFound(version));
        }

        let mesh = Self::load_mesh_from_file(&version_path)?;
        self.current_mesh = mesh;
        Ok(())
    }

    /// Call before application exit to ensure final save
    pub fn save_on_exit(&mut self) -> Result<(), MeshDocumentError> {
        if self.has_unsaved_changes() {
            self.save_current()?;
        }
        Ok(())
    }

    fn load_mesh_from_file(path: &Path) -> Result<Mesh, MeshDocumentError> {
        let contents = fs::read_to_string(path).map_err(|e| MeshDocumentError::ReadError {
            path: path.to_path_buf(),
            source: e,
        })?;

        let mesh: Mesh = ron::from_str(&contents).map_err(|e| MeshDocumentError::ParseError {
            path: path.to_path_buf(),
            source: e,
        })?;

        Ok(mesh)
    }

    fn save_mesh_to_file(&self, mesh: &Mesh, path: &Path) -> Result<(), MeshDocumentError> {
        let contents = ron::ser::to_string_pretty(mesh, Default::default())?;

        fs::write(path, contents).map_err(|e| MeshDocumentError::WriteError {
            path: path.to_path_buf(),
            source: e,
        })?;

        Ok(())
    }

    fn find_next_version_number(&self) -> Result<i32, MeshDocumentError> {
        let mut existing_versions: HashSet<i32> = HashSet::new();

        let entries =
            fs::read_dir(&self.folder_path).map_err(|e| MeshDocumentError::ReadError {
                path: self.folder_path.clone(),
                source: e,
            })?;

        for entry in entries {
            let entry = entry.map_err(|e| MeshDocumentError::ReadError {
                path: self.folder_path.clone(),
                source: e,
            })?;

            let name = entry.file_name();
            if let Some(name_str) = name.to_str() {
                if let Some(num_str) = name_str
                    .strip_prefix("version_")
                    .and_then(|s| s.strip_suffix(".mesh"))
                {
                    if let Ok(num) = num_str.parse::<i32>() {
                        existing_versions.insert(num);
                    }
                }
            }
        }

        // Find lowest available number starting from 1
        let mut next = 1;
        while existing_versions.contains(&next) {
            next += 1;
        }
        Ok(next)
    }
}

/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::error::{ModuleError::*, Result};

use fluence_app_service::{ConfigContext, ModuleDescriptor, TomlMarineNamedModuleConfig};
use service_modules::{
    blueprint_file_name, blueprint_fname, module_config_name_hash, module_file_name_hash,
    Blueprint, Hash,
};

use std::convert::TryInto;
use std::path::Path;

/// Load blueprint from disk
pub fn load_blueprint(bp_dir: &Path, blueprint_id: &str) -> Result<Blueprint> {
    let bp_path = bp_dir.join(blueprint_fname(blueprint_id));
    let blueprint =
        std::fs::read(&bp_path).map_err(|err| NoSuchBlueprint { path: bp_path, err })?;
    let blueprint: Blueprint =
        toml::de::from_slice(blueprint.as_slice()).map_err(|err| IncorrectBlueprint { err })?;

    Ok(blueprint)
}

/// Load ModuleDescriptor from disk for a given module name
pub fn load_module_descriptor(modules_dir: &Path, module_hash: &Hash) -> Result<ModuleDescriptor> {
    let config = modules_dir.join(module_config_name_hash(module_hash));
    let config = load_config_by_path(&config)?;
    // `base_path: None` tells Marine to resolve non-absolute paths relative to the current directory
    let context = ConfigContext { base_path: None };

    let mut config: ModuleDescriptor = context
        .wrapped(config)
        .try_into()
        .map_err(|err| ModuleConvertError { err })?;

    // TODO HACK: This is required because by default file_name is set to be same as import_name
    //            That behavior is defined in TomlMarineNamedModuleConfig. Would be nice to refactor that behavior.
    config.file_name = module_file_name_hash(module_hash);

    Ok(config)
}

/// Load TomlMarineNamedModuleConfig from disk from a given path
pub fn load_config_by_path(path: &Path) -> Result<TomlMarineNamedModuleConfig> {
    let config = std::fs::read(path).map_err(|err| NoModuleConfig {
        path: path.to_path_buf(),
        err,
    })?;
    let config: TomlMarineNamedModuleConfig =
        toml::de::from_slice(config.as_slice()).map_err(|err| IncorrectModuleConfig { err })?;

    Ok(config)
}

/// Adds a module to the filesystem, overwriting existing module.
/// Also adds module config to the TomlMarineNamedModuleConfig
pub fn add_module(
    modules_dir: &Path,
    module_hash: &Hash,
    bytes: &[u8],
    mut config: TomlMarineNamedModuleConfig,
) -> Result<TomlMarineNamedModuleConfig> {
    let wasm = modules_dir.join(module_file_name_hash(module_hash));
    std::fs::write(&wasm, bytes).map_err(|err| AddModule { path: wasm, err })?;

    // replace existing configuration with a new one
    // TODO HACK: use custom structure for API; TomlMarineNamedModuleConfig is too powerful and clumsy.
    // Set file_name = ${hash}.wasm
    config.file_name = Some(module_file_name_hash(module_hash));
    // The `load_from` field overrides `modules_dir` for a single module,
    // so we set `load_from` to `None`, telling Marine to load modules from the `modules_dir`
    config.load_from = None;

    let toml = toml::to_string_pretty(&config).map_err(|err| SerializeConfig {
        err,
        config: config.clone(),
    })?;
    let path = modules_dir.join(module_config_name_hash(module_hash));
    std::fs::write(&path, toml).map_err(|err| WriteConfig { path, err })?;

    Ok(config)
}

pub fn load_module_by_path(path: &Path) -> Result<Vec<u8>> {
    std::fs::read(path).map_err(|err| ModuleNotFound {
        path: path.to_path_buf(),
        err,
    })
}

/// Saves new blueprint to disk
pub fn add_blueprint(blueprint_dir: &Path, blueprint: &Blueprint) -> Result<()> {
    let path = blueprint_dir.join(blueprint_file_name(blueprint));

    // Save blueprint to disk
    let bytes = toml::ser::to_vec(&blueprint).map_err(|err| SerializeBlueprint {
        err,
        blueprint: blueprint.clone(),
    })?;
    std::fs::write(&path, bytes).map_err(|err| WriteBlueprint { path, err })?;

    // TODO: check dependencies are satisfied?

    Ok(())
}

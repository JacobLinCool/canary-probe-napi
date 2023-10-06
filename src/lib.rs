#![deny(clippy::all)]

use canary_probe::{bollard::Docker, run_checks, CheckConfig, CheckError};
use napi::{Error, Result};
use napi_derive::*;

#[napi(object)]
#[derive(Debug)]
pub struct JsCheckConfig {
    pub image: Option<String>,
    pub hostname: Option<String>,
    pub working_dir: Option<String>,
    pub zip_name: Option<String>,
    pub timeout: Option<i64>,
    pub memory_limit: Option<i64>,
    pub cpu_limit: Option<i64>,
    pub disk_limit: Option<String>,
    pub extract: Option<String>,
    pub debug: Option<bool>,
}

#[napi(object)]
#[derive(Debug)]
pub struct JsCheckResult {
    pub executables: Vec<String>,
}

/// Check the archive at the given path is ok to unzip, build, and contains executables
#[napi]
pub async fn check(zip_path: String, option: Option<JsCheckConfig>) -> Result<JsCheckResult> {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let mut config = CheckConfig::default();
    if let Some(option) = option {
        if let Some(image) = option.image {
            config.image = image;
        }
        if let Some(hostname) = option.hostname {
            config.hostname = hostname;
        }
        if let Some(working_dir) = option.working_dir {
            config.working_dir = working_dir;
        }
        if let Some(zip_name) = option.zip_name {
            config.zip_name = zip_name;
        }
        if let Some(timeout) = option.timeout {
            config.timeout = timeout;
        }
        if let Some(memory_limit) = option.memory_limit {
            config.memory_limit = memory_limit;
        }
        if let Some(cpu_limit) = option.cpu_limit {
            config.cpu_limit = cpu_limit;
        }
        if let Some(disk_limit) = option.disk_limit {
            config.disk_limit = disk_limit;
        }
        if let Some(extract) = option.extract {
            config.extract = Some(extract);
        }
        if let Some(debug) = option.debug {
            config.debug = debug;
        }
    }

    let result = run_checks(&docker, &zip_path, config).await;

    match result {
        Ok(executables) => Ok(JsCheckResult { executables }),
        Err(e) => {
            match e.downcast_ref() {
                Some(CheckError::ImagePullError) => Err(Error::from_reason("Failed to pull image")),
                Some(CheckError::ContainerCreateError { output }) => Err(Error::from_reason(
                    format!("Failed to create container: {}", output),
                )),
                Some(CheckError::ContainerStartError { output }) => Err(Error::from_reason(
                    format!("Failed to start container: {}", output),
                )),
                Some(CheckError::UnzipError { output }) => {
                    Err(Error::from_reason(format!("Failed to unzip: {}", output)))
                }
                Some(CheckError::MakeError { output }) => {
                    Err(Error::from_reason(format!("Failed to make: {}", output)))
                }
                Some(CheckError::FindError { output }) => Err(Error::from_reason(format!(
                    "Failed to find executables: {}",
                    output
                ))),
                _ => Err(Error::from_reason(format!("Unknown error: {:?}", e))),
            }
        }
    }
}

/// Check if docker is available
#[napi]
pub async fn available() -> Result<bool> {
    let docker = Docker::connect_with_local_defaults();
    if docker.is_err() {
        return Ok(false);
    }
    let docker = docker.unwrap();

    let info = docker.info().await;
    if info.is_err() {
        return Ok(false);
    }

    Ok(true)
}

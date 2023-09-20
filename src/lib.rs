#![deny(clippy::all)]

use canary_probe::{bollard::Docker, run_checks, CheckConfig, CheckError};
use napi::{Error, Result};
use napi_derive::*;

#[napi]
pub async fn check(absolute_zip_path: String) -> Result<Vec<String>> {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let config = CheckConfig::default();

    let result = run_checks(&docker, &absolute_zip_path, config).await;

    match result {
        Ok(executables) => Ok(executables),
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

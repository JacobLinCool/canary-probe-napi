/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface JsCheckConfig {
  image?: string
  hostname?: string
  workingDir?: string
  zipName?: string
  timeout?: number
  memoryLimit?: number
  cpuLimit?: number
  diskLimit?: string
  extract?: string
  debug?: boolean
}
export interface JsCheckResult {
  executables: Array<string>
}
/** Check the archive at the given path is ok to unzip, build, and contains executables */
export function check(zipPath: string, option?: JsCheckConfig | undefined | null): Promise<JsCheckResult>
/** Check if docker is available */
export function available(): Promise<boolean>

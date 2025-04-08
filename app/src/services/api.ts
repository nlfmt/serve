import { DirInfo, EntryProperties as EntryProperties } from "@/util/models"
import { Result } from "@/util/result"
import axios, { AxiosError, AxiosProgressEvent } from "axios"

const API_URL = `http://${location.host}/api`

async function fetchFiles(path: string): Promise<Result<DirInfo, string>> {
  const query = new URLSearchParams({ path })
  const res = await fetch(API_URL + `/files?${query.toString()}`, {})
  if (!res.ok) {
    const err = await res.text().catch(e => String(e))
    return Result.Err(err)
  }

  try {
    const data = await res.json()
    return Result.Ok(data)
  } catch (_e) {
    return Result.Err("Could not parse JSON")
  }
}

async function uploadFile(
  path: string,
  file: File,
  overwrite: boolean,
  signal?: AbortSignal,
  onUploadProgress?: (p: AxiosProgressEvent) => void,
): Promise<Result<undefined, { code: number; text: string }>> {
  const query = new URLSearchParams({
    path,
    file_name: file.name,
    overwrite: String(!!overwrite),
  })
  const url = API_URL + `/upload?${query.toString()}`

  try {
    await axios.head(url)

    await axios.post(url, file, {
      headers: {
        "Content-Type": "multipart/form-data",
      },
      onUploadProgress,
      signal,
    })

    return Result.Ok(undefined)
  } catch (e: unknown) {
    const code = (e as AxiosError)?.status ?? 500
    switch (code) {
      case 409:
        return Result.Err({ code, text: "File Already Exists" })
      case 400:
        return Result.Err({ code, text: "Invalid Path" })
      default:
        return Result.Err({ code, text: "File Upload failed" })
    }
  }
}

export interface Settings {
  upload: boolean
  allow_rename: boolean
  allow_delete: boolean
}

async function getSettings(): Promise<Result<Settings, string>> {
  return await axios
    .get(API_URL + "/settings")
    .then(res => {
      return Result.Ok(res.data as Settings)
    })
    .catch((e: AxiosError) => {
      return Result.Err(String(e.response?.data) || e.message)
    })
}

async function getEntryProperties(
  path: string,
): Promise<Result<EntryProperties, string>> {
  const query = new URLSearchParams({ path })
  return await axios
    .get(API_URL + "/properties?" + query.toString())
    .then(res => {
      return Result.Ok(res.data as EntryProperties)
    })
    .catch((e: AxiosError) => {
      return Result.Err(String(e.response?.data) || e.message)
    })
}

async function rename(path: string, to: string): Promise<Result<null, string>> {
  return await axios
    .put(API_URL + "/rename", { path, to })
    .then(() => {
      return Result.Ok(null)
    })
    .catch((e: AxiosError) => {
      return Result.Err(String(e.response?.data) || e.message)
    })
}

async function remove(path: string): Promise<Result<null, string>> {
  const query = new URLSearchParams({ path })
  return await axios
    .delete(API_URL + "/delete?" + query.toString())
    .then(() => {
      return Result.Ok(null)
    })
    .catch((e: AxiosError) => {
      return Result.Err(String(e.response?.data) || e.message)
    })
}

async function downloadFolder(path: string) {
  const query = new URLSearchParams({ path })
  const a = document.createElement("a")
  a.href = `${API_URL}/download_folder?${query.toString()}`
  a.download = path.split("/").at(-1) ?? "folder.zip"
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
}

export default {
  fetchFiles,
  uploadFile,
  getSettings,
  rename,
  remove,
  getEntryProperties,
  downloadFolder,
}

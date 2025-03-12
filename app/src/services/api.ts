import { DirInfo } from "@/util/models"
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
    } catch(_e) {
        return Result.Err("Could not parse JSON")
    }
}

async function uploadFile(
  path: string,
  file: File,
  overwrite: boolean,
  signal?: AbortSignal,
  onUploadProgress?: (p: AxiosProgressEvent) => void,
): Promise<Result<undefined, { code: number, text: string }>> {
  const query = new URLSearchParams({
    path,
    file_name: file.name,
    overwrite: String(!!overwrite),
  })
  const body = new FormData()
  body.append("file", file)
  const url = API_URL + `/upload?${query.toString()}`

  try {
    await axios.head(url)

    await axios.post(url, body, {
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
      case 409: return Result.Err({ code, text: "File Already Exists" })
      case 400: return Result.Err({ code, text: "Invalid Path" })
      default: return Result.Err({ code, text: "File Upload failed" })
    }
  }
}

async function isUploadEnabled(): Promise<Result<boolean, string>> {
    return await axios.get(API_URL + "/upload_enabled").then(res => {
        console.log(res.data)
        return Result.Ok<boolean, string>(res.data === true)
    }).catch((e: AxiosError) => {
        return Result.Err(String(e.response?.data) || e.message)
    })
}

export default {
    fetchFiles,
    uploadFile,
    isUploadEnabled,
}
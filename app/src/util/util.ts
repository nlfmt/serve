import { DirInfo } from "./models"

export function sizeString(bytes: number) {
  let i = 0
  const units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"]

  while (bytes > 1024) {
    bytes /= 1024
    i++
  }

  return `${bytes.toPrecision(3)} ${units[i]}`
}

export function pluralString(count: number, name: string, plural: string) {
  return `${count}  ${count === 1 ? name : plural}`
}

export function dirInfo(data: DirInfo) {
  const fc = data.files.length
  const dc = data.dirs.length
  const c = fc + dc
  return `${pluralString(c, "entry", "entries")} - ${pluralString(
    dc,
    "folder",
    "folders",
  )}, ${pluralString(fc, "file", "files")}`
}

export function removeTrailingSlash(path: string) {
  return path.replace(/\/$/, "")
}

export function joinPath(...segments: string[]) {
  return segments.join("/").replace(/\/\//, "/")
}

export function durationString(s: number) {
  s = Math.round(s)
  const h = Math.floor(s / 3600)
  s -= h * 3600
  const m = Math.floor(s / 60)
  s -= m * 60
  return (h > 0 ? `${h}h ` : "") + (m > 0 ? `${m}m ` : "") + (h == 0 ? `${s}s` : "")
}
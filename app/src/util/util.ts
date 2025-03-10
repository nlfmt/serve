import { DirInfo } from "./models"

export function getReadableFileSizeString(fileSizeInBytes: number) {
  let i = 0
  const byteUnits = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"]

  while (fileSizeInBytes > 1024) {
    fileSizeInBytes /= 1024
    i++
  }

  return `${fileSizeInBytes.toFixed(0)} ${byteUnits[i]}`
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

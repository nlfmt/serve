import { EntryProperties, FileInfo, FolderInfo } from "@/util/models"
import c from "./InfoModal.module.scss"
import { ReactNode, useEffect, useMemo, useState } from "react";
import api from "@/services/api";
import { joinPath, sizeString } from "@/util/util";
import { FolderOutlined, ReportGmailerrorredOutlined } from "@mui/icons-material";
import FileIcon from "../FileIcon/FileIcon";
import { DateTime } from "luxon";

function boolStr(value: boolean | undefined) {
  return value === undefined ? value : value ? "yes" : "no"
}

function InfoModal({ path, info, isDir }: { path: string; info: FileInfo | FolderInfo, isDir: boolean }) {
  const [properties, setProperties] = useState<EntryProperties | null>(null)
  const [error, setError] = useState("")
  
  const timestamps = useMemo(() => {
    return {
      accessed: properties?.accessed ? DateTime.fromSeconds(properties?.accessed) : undefined,
      modified: info.modified ? DateTime.fromSeconds(info.modified) : undefined,
      created: info.created ? DateTime.fromSeconds(info.created) : undefined,
    }
  }, [properties?.accessed, info])

  useEffect(() => {
    api.getEntryProperties(joinPath(path, info.name)).then(res => {
      if (res.ok) {
        setProperties(res.value)
      } else {
        setError(res.error)
      }
    })
  }, [path, info.name])

  return (
    <div className={c.container}>
      <div className={c.header}>
        <div className={c.icon}>
          {isDir ? <FolderOutlined /> : <FileIcon file={info.name} />}
        </div>
        <div className={c.info}>
          <span className={c.name} title={info.name}>
            {info.name}
          </span>
          <span className={c.dark}>
            {info.is_symlink ? "Link to " : ""}{isDir ? "Folder" : "File"}
          </span>
        </div>
      </div>
      {error && (
        <div className={c.error}>
          <ReportGmailerrorredOutlined />
          <span>{error}</span>
        </div>
      )}
      <div className={c.content}>
        <div className={c.divider} />
        <Property label="Location" value={path} />
        <Property
          label="Size"
          value={"size" in info ? sizeString(info.size) : "-"}
        />
        <div className={c.divider} />
        <Property
          label="Created At"
          value={timestamps.created?.toLocaleString()}
        />
        <Property
          label="Last Modified"
          value={timestamps.modified?.toLocaleString()}
        />
        <Property
          label="Last Accessed"
          value={timestamps?.accessed?.toLocaleString()}
        />
        <div className={c.divider} />
        <Property label="Symlink" value={boolStr(info.is_symlink)} />
        <Property label="Readonly" value={boolStr(properties?.readonly)} />
      </div>
    </div>
  )
}

function Property({ label, value }: { label: string, value: ReactNode | undefined }) {
  return (
    <span className={c.property}>
      <label>{label}</label>
      <span>{value ?? "loading.."}</span>
    </span>
  )
}

export default InfoModal

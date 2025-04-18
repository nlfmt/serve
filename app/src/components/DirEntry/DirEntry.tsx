import { DragEvent, ReactNode, useMemo } from "react"
import { DateTime } from "luxon"
import { joinPath, sizeString } from "@/util/util"
import c from "./DirEntry.module.scss"
import { classes } from "@/util/classes"
import useDropTarget from "@/hooks/useDropTarget"
import common from "../../styles/common.module.scss"
import { LinkOutlined } from "@mui/icons-material"
import { useNavigation } from "@/hooks/useNavigation"
import { useSettings } from "@/hooks/useSettings"

namespace DirEntry {
  export interface Props {
    onClick?: () => void
    icon: ReactNode
    children?: ReactNode
    info: {
      name: string
      size?: number
      modified?: number
      created?: number
      is_symlink: boolean
    }
    download?: string
    onDrop?: (e: DragEvent) => void
  }
}

function DirEntry({ info, icon, onClick, children, download, onDrop }: DirEntry.Props) {
  const { path } = useNavigation()
  const settings = useSettings()
  const [modified, created] = useMemo(() => {
    return [
      info.modified ? DateTime.fromSeconds(info.modified) : null,
      info.created ? DateTime.fromSeconds(info.created) : null,
    ]
  }, [info.modified, info.created])

  const { dropHover, dropTargetProps } = useDropTarget({
    onDrop,
  })

  return (
    <a
      target="_blank"
      className={classes(c.entry, [c.dropHover, dropHover])}
      href={download}
      download={!!download}
      draggable={settings.rename}
      onDragStart={e => {
        if (!settings.rename) return
        // setup for moving file/folder
        e.dataTransfer.clearData()
        e.dataTransfer.setData("path", joinPath(path, info.name))
      }}
      onClick={e => {
        if (e.defaultPrevented) return
        onClick?.()
      }}
      {...(onDrop ? dropTargetProps : [])}
    >
      <div className={c.icon}>{icon}</div>
      <span className={c.fileName} title={info.name}>
        <span className={c.fileNameLabel}>{info.name}</span>
        {info.is_symlink && <LinkOutlined />}
      </span>
      <span className={c.info}>{info.size ? sizeString(info.size) : "-"}</span>
      <Timestamp className={common.hideOnMobile} value={modified} />
      <Timestamp className={common.hideOnTablet} value={created} />
      <div className={c.actions}>{children}</div>
    </a>
  )
}

function Timestamp({
  value,
  className,
}: {
  value: DateTime<true> | null
  className?: string
  style?: React.HTMLAttributes<HTMLDivElement>["style"]
}) {
  return (
    <span
      className={classes(c.info, className)}
      title={value?.toLocaleString(DateTime.DATETIME_FULL_WITH_SECONDS)}
    >
      {value?.toLocaleString()}
    </span>
  )
}

export default DirEntry

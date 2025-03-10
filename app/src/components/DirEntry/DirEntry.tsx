import { ReactNode, useMemo } from "react"
import { DateTime } from "luxon"
import { getReadableFileSizeString } from "@/util/util"
import c from "./DirEntry.module.scss"
import { classes } from "@/util/classes"

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
    }
    download?: string
  }
}

function DirEntry({ info, icon, onClick, children, download }: DirEntry.Props) {
  const [modified, created] = useMemo(() => {
    return [
      info.modified ? DateTime.fromSeconds(info.modified) : null,
      info.created ? DateTime.fromSeconds(info.created) : null,
    ]
  }, [info.modified, info.created])

  return (
    <a target="_blank" className={c.entry} href={download} download={!!download} onClick={onClick}>
      <div className={c.icon}>
        {icon}
      </div>
      <span>{info.name}</span>
      <span className={c.info}>
        {info.size ? getReadableFileSizeString(info.size) : "-"}
      </span>
      <Timestamp className={c.hideOnMobile} value={modified} />
      <Timestamp className={c.hideOnTablet} value={created} />
      <div className={c.actions}>
        {children}
      </div>
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

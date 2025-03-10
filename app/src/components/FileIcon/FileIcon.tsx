import { useEffect, useState } from "react"

import fileIcons from "@exuanbo/file-icons-js"
import { ArticleOutlined } from "@mui/icons-material"

export namespace FileIcon {
  export interface Props {
    file: string
  }
}

function FileIcon({ file }: FileIcon.Props) {
  const [iconClass, setIconsClass] = useState<string | null>(null)

  useEffect(() => {
    fileIcons.getClass(file).then((cls: string) => {
      setIconsClass(cls)
    })
  }, [file])

  return iconClass ? <i className={iconClass} /> : <ArticleOutlined />
}

export default FileIcon

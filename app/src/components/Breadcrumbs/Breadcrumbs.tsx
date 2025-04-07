import { useMemo } from "react"
import c from "./Breadcrumbs.module.scss"
import { FolderZipOutlined, HomeRounded, QrCodeOutlined } from "@mui/icons-material"
import { useNavigation } from "@/hooks/useNavigation"
import ContextMenu from "../ContextMenu/ContextMenu"

function Breadcrumbs() {
  const { path } = useNavigation()

  const segments = useMemo(() => {
    const segments = path.match(/\/|([^/]+)/g)
    return segments ? [...segments] : []
  }, [path])

  return (
    <div className={c.breadcrumbs}>
      <div
        className={c.homeIcon}
        onClick={() => window.history.pushState(null, "", "/")}
      >
        <HomeRounded sx={{ fontSize: 20 }} />
      </div>

      <div className={c.path}>
        {segments.map((s, i) => (
          <span
            key={i}
            className={s === "/" ? c.pathSlash : c.pathSegment}
            onClick={
              s !== "/"
                ? () => {
                    window.history.pushState(
                      null,
                      "",
                      segments.slice(0, i + 1).join(""),
                    )
                  }
                : undefined
            }
          >
            {s}
          </span>
        ))}
      </div>
      
      {/* <a download target="_blank" href={downloadUrl} className={c.downloadButton}>
      <Button variant="primary">Download Folder</Button>
      </a> */}
      <ContextMenu className={c.contextMenu}>
        <ContextMenu.Item label="Show QR Code" icon={<QrCodeOutlined />} />
        <ContextMenu.Item label="Download ZIP" icon={<FolderZipOutlined />} />
        <ContextMenu.Item label="hallo" />
      </ContextMenu>
    </div>
  )
}

export default Breadcrumbs

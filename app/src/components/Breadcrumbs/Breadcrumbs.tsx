import { useMemo } from "react"
import c from "./Breadcrumbs.module.scss"
import { HomeRounded } from "@mui/icons-material"
import { useNavigation } from "@/hooks/useNavigation"
import Button from "../Button/Button"

function Breadcrumbs() {
  const { path } = useNavigation()

  const downloadUrl = useMemo(() => {
    const params = new URLSearchParams({
      path
    })
    return `/api/download_folder?${params.toString()}`
  }, [path])

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
      
      <a download target="_blank" href={downloadUrl} className={c.downloadButton}>
      <Button variant="primary">Download Folder</Button>
      </a>
    </div>
  )
}

export default Breadcrumbs

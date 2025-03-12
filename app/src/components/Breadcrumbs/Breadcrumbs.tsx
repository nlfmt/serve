import { useMemo } from "react"
import c from "./Breadcrumbs.module.scss"
import { HomeRounded } from "@mui/icons-material"
import { useNavigation } from "@/hooks/useNavigation"

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
    </div>
  )
}

export default Breadcrumbs

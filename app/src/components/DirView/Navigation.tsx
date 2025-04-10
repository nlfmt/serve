import { useNavigation } from "@/hooks/useNavigation"
import c from "./Navigation.module.scss"
import { ArrowBackRounded, ArrowForwardRounded, ArrowUpwardRounded, ReplayRounded } from "@mui/icons-material"
import { classes } from "@/util/classes"

namespace Navigation {
    export interface Props {
        reload: () => void
        loading: boolean
        className?: string
    }
}

function Navigation(props: Navigation.Props) {
    const { up, prev, next } = useNavigation()

    return (
        <div className={classes(c.navigation, props.className)}>
          <div
            title="Up one level"
            className={c.navigationButton}
            onClick={up}
          >
            <ArrowUpwardRounded />
          </div>
          <div
            title="Previous folder"
            className={c.navigationButton}
            onClick={prev}
          >
            <ArrowBackRounded />
          </div>
          <div
            title="Next folder"
            className={c.navigationButton}
            onClick={next}
          >
            <ArrowForwardRounded />
          </div>
          <div
            title="Reload Folder"
            className={classes(c.navigationButton, [c.rotate, props.loading])}
            onClick={() => {
              if (!props.loading) props.reload()
            }}
          >
            <ReplayRounded />
          </div>
        </div>
    )
}

export default Navigation
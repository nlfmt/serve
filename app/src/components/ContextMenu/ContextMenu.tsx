import {
  createContext,
  ReactElement,
  ReactNode,
  useContext,
  useId,
  useRef,
} from "react"
import c from "./ContextMenu.module.scss"
import { MoreHoriz } from "@mui/icons-material"
import Button from "../Button/Button"
import { classes } from "@/util/classes"

type Context = { close: () => void }
const Context = createContext<Context | null>(null)
type MaybeArray<T> = T | T[]

namespace ContextMenu {
  export interface Props {
    className?: string
    children: MaybeArray<ReactElement<ContextMenu.Item.Props> | false | null>
  }
}

function ContextMenu(props: ContextMenu.Props) {
  const menuId = useId()
  const buttonId = useId()
  const ref = useRef<HTMLDivElement>(null)
  const anchorId = `--anchor-${buttonId.replace(/[^a-zA-Z0-9-]/g, "")}`

  function close() {
    ref.current?.hidePopover()
  }

  // useEffect(() => {
  //   if (!ref.current) return
  //   ref.current.["anchor-name"] = _id
  // }, [buttonId, _id])

  return (
    <Context.Provider value={{ close }}>
      <Button
        id={buttonId}
        style={{ "--anchor-id": anchorId } as React.CSSProperties}
        size="small"
        variant="secondary"
        className={classes(c.button, props.className)}
        popoverTarget={menuId}
        onClick={e => {
          e.preventDefault()
          ref.current?.togglePopover()
        }}
      >
        <MoreHoriz className={c.moreIcon} />
      </Button>
      <div
        style={{ "--anchor-id": anchorId } as React.CSSProperties}
        ref={ref}
        id={menuId}
        popover="auto"
        className={c.contextmenu}
        onClick={e => e.preventDefault()}
      >
        {props.children}
      </div>
    </Context.Provider>
  )
}

namespace ContextMenu {
  export namespace Item {
    export interface Props {
      onClick?: () => void
      label: string
      icon?: ReactNode
    }
  }

  export function Item({ onClick, label, icon }: Item.Props) {
    const { close } = useContext(Context)!

    return (
      <div
        className={c.item}
        onClick={() => {
          close()
          onClick?.()
        }}
      >
        <span className={c.label}>{label}</span>
        {icon && <div className={c.icon}>{icon}</div>}
      </div>
    )
  }
}

export default ContextMenu

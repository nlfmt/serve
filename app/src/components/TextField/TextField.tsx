import { ComponentProps, ReactNode } from "react"
import c from "./TextField.module.scss"
import { classes } from "@/util/classes"

namespace TextField {
  export interface Props extends ComponentProps<"input"> {
    icon?: ReactNode
  }
}

function TextField({ value, onChange, icon, className, ...props }: TextField.Props) {
  return (
    <div className={classes(c.textField, className)}>
      {icon}
      <input
        type="text"
        placeholder="Search"
        value={value}
        onChange={onChange}
        {...props}
      />
    </div>
  )
}

export default TextField

import { ComponentProps } from "react"
import { classes } from "../../util/classes"
import c from "./Button.module.scss"

namespace Button {
  export interface Props extends ComponentProps<"button"> {
    variant?: "primary" | "secondary"
    size?: "normal" | "small" | "large"
  }
}

function Button({
  className,
  variant = "primary",
  size = "normal",
  ...props
}: Button.Props) {
  return (
    <button
      data-variant={variant}
      data-size={size}
      className={classes(c.button, className)}
      {...props}
    >
      {props.children}
    </button>
  )
}

export default Button

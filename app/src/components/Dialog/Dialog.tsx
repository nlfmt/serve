import { classes } from "@/util/classes";
import { ComponentProps } from "react";
import c from "./Dialog.module.scss"

function Dialog({className, ...props}: ComponentProps<"dialog">) {
    return (
        <dialog className={classes(className, c.dialog)} {...props}>
            {props.children}
        </dialog>
    )
}

export default Dialog
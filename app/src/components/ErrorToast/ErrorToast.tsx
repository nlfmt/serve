import useToastContext from "@/hooks/useToastContext"
import { useEffect } from "react"

import c from "./ErrorToast.module.scss"

namespace ErrorToast {
  export interface Props {
    error: string
  }
}

function ErrorToast(props: ErrorToast.Props) {
  const { remove } = useToastContext()

  useEffect(() => {
    const id = setTimeout(remove, 5000)
    return () => clearTimeout(id)
  }, [remove])

  return <div className={c.container}>{props.error}</div>
}

export default ErrorToast

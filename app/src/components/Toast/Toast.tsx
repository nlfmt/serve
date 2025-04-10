import useToastContext from "@/hooks/useToastContext"
import { ReactNode, useEffect } from "react"

import c from "./Toast.module.scss"
import { CheckCircleOutlineOutlined, ErrorOutlineOutlined, InfoOutlined } from "@mui/icons-material"

namespace Toast {
  export interface Props {
    text: string
    icon?: ReactNode,
    type?: "default" | "error" | "success"
    timeout?: number
  }
}

function Toast({ text, icon, type = "default", timeout = 5000 }: Toast.Props) {
  const { remove } = useToastContext()

  useEffect(() => {
    const id = setTimeout(remove, timeout)
    return () => clearTimeout(id)
  }, [remove, timeout])

  return (
    <div data-type={type} className={c.container}>
      {icon ||
        (type == "success" ? (
          <CheckCircleOutlineOutlined />
        ) : type == "error" ? (
          <ErrorOutlineOutlined />
        ) : (
          <InfoOutlined />
        ))}
      <span>{text}</span>
    </div>
  )
}

export default Toast

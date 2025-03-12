import { ToastContext } from "@/contexts/Toast.context"
import { ToastService, ToastServiceContext } from "@/contexts/ToastService.context"
import { ReactNode, useCallback, useState } from "react"
import c from "./ToastManager.module.scss"

namespace ToastManager {
  export interface Props {
    children?: ReactNode
  }
}

export interface ToastProps {
  remove: () => void
}

function getId() {
  return Date.now().toString(36) + Math.random().toString(36).substring(2)
}

function ToastManager(props: ToastManager.Props) {
  const [toasts, setToasts] = useState<{
    id: string,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    Toast: React.ComponentType<any>,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    props: any,
    state: string
  }[]>([])
  
  const remove = useCallback((id: string) => {
    setToasts(toasts => {
      return toasts.map(t => {
        if (t.id !== id) return t
        return { ...t, state: "leave" }
      })
    })
    setTimeout(() => {
      setToasts(toasts => toasts.filter(t => t.id !== id))
    }, 300)
  }, [])

  const add = useCallback<ToastService["add"]>((Toast, props) => {
    const id = getId()
    setToasts(toasts => {
      return [...toasts, { id, Toast, props, state: "enter" }]
    })
  }, [])
  return (
    <ToastServiceContext.Provider value={{
      add,
    }}>
      {props.children}
      <div className={c.toasts}>
        {toasts.map(({ id, Toast, props, state }) => (
          <ToastContext.Provider key={id} value={{ remove: () => remove(id) }}>
            <div data-state={state} className={c.toast}>
              <Toast {...props} />
            </div>
          </ToastContext.Provider>
        ))}
      </div>
    </ToastServiceContext.Provider>
  )
}

export default ToastManager
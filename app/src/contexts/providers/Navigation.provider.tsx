import { ReactNode, useCallback, useEffect, useRef, useState } from "react"
import { NavigationContext } from "../Navigation.context"
import { joinPath, removeTrailingSlash } from "@/util/util"

export type Navigation = ReturnType<typeof useNavigationLogic>

const legacy = !window.navigation

function useNavigationLogic(initial: string) {
  const [path, setPath] = useState(initial)
  const isNavigating = useRef(false)

  function _navigate(path: string) {
    window.history.pushState(null, "", path)
    // for firefox (🤮)
    if (legacy) setPath(path)
  }

  useEffect(() => {
    const ctrl = new AbortController()

    window.navigation?.addEventListener(
      "navigate",
      e => {
        if (e.destination.url.includes("?path=")) return
        setPath(new URL(e.destination.url).pathname ?? "/")
        isNavigating.current = false
      },
      { signal: ctrl.signal },
    )

    return () => ctrl.abort()
  }, [])

  const navigate = useCallback(
    (folder: string) => {
      if (isNavigating.current) return
      if (!legacy) isNavigating.current = true
      _navigate(joinPath(path, folder))
    },
    [path, isNavigating],
  )

  const up = useCallback(() => {
    if (path === "/" || isNavigating.current) return
    isNavigating.current = true
    const newPath = removeTrailingSlash(path).split("/").slice(0, -1).join("/")
    _navigate(newPath || "/")
  }, [path, isNavigating])

  const home = () => {
    _navigate("/")
  }
  
  const prev = () => {
    if (legacy) {
      window.addEventListener(
        "popstate",
        () => setPath(window.location.pathname),
        { once: true },
      )
    }
    window.history.back()
  }

  const next = () => {
    if (legacy) {
      window.addEventListener(
        "popstate",
        () => setPath(window.location.pathname),
        { once: true },
      )
    }
    window.history.forward()
  }

  return { path, navigate, up, home, prev, next }
}

export function NavigationProvider(props: { children: ReactNode }) {
  const navigation = useNavigationLogic(location.pathname)

  return (
    <NavigationContext.Provider value={navigation}>
      {props.children}
    </NavigationContext.Provider>
  )
}

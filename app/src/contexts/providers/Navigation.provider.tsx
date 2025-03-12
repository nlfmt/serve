import { ReactNode, useCallback, useEffect, useRef, useState } from "react"
import { NavigationContext } from "../Navigation.context"
import { joinPath, removeTrailingSlash } from "@/util/util"

export type Navigation = ReturnType<typeof useNavigationLogic>

function useNavigationLogic(initial: string) {
  const [path, setPath] = useState(initial)
  const isNavigating = useRef(false)

  useEffect(() => {
    const ctrl = new AbortController()

    window.navigation.addEventListener(
      "navigate",
      e => {
        if (e.destination.url.includes("api/download?path=")) return
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
      isNavigating.current = true
      window.history.pushState(null, "", joinPath(path, folder))
    },
    [path, isNavigating],
  )

  const up = useCallback(() => {
    if (path === "/" || isNavigating.current) return
    isNavigating.current = true
    const newPath = removeTrailingSlash(path).split("/").slice(0, -1).join("/")
    window.history.pushState(null, "", newPath || "/")
  }, [path, isNavigating])

  return { path, navigate, up }
}

export function NavigationProvider(props: { children: ReactNode }) {
  const navigation = useNavigationLogic(location.pathname)

  return (
    <NavigationContext.Provider value={navigation}>
      {props.children}
    </NavigationContext.Provider>
  )
}

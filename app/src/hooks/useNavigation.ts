import { useCallback, useEffect, useRef, useState } from "react"

function removeTrailingSlash(path: string) {
  return path.replace(/\/$/, "")
}

function removeStartingSlash(path: string) {
  return path.replace(/^\//, "")
}

export default function useNavigation(initial: string) {
  const [path, setPath] = useState(initial)
  // const [isNavigating, setIsNavigating] = useState(false)
  const isNavigating = useRef(false)

  useEffect(() => {
    const ctrl = new AbortController()

    window.navigation.addEventListener(
      "navigate",
      e => {
        if (e.destination.url.includes("api/download?path=")) return
        console.log(`navigate, destination: ${new URL(e.destination.url).pathname}`)
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
      const newPath = path === "/" ? path + folder : path + "/" + folder
      console.log(`navigating to ${newPath}, ${removeStartingSlash(newPath)}, path: ${path}, folder: ${folder}`)
      window.history.pushState(null, "", newPath)
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

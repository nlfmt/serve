import { DragEvent, useCallback, useRef, useState } from "react";


export default function useDropTarget(opts: { allowed?: (e: DragEvent) => boolean, onDrop?: (e: DragEvent) => void }, deps?: React.DependencyList) {
    const count = useRef(0)
    const [dropHover, setDropHover] = useState(false)
    
    function onDragEnter(e: DragEvent) {
        e.preventDefault()
        e.stopPropagation()
        if (opts.allowed && !opts.allowed(e)) return
        count.current += 1
        if (!dropHover && count.current > 0) setDropHover(true)
    }

    function onDragLeave(e: DragEvent) {
        e.preventDefault()
        e.stopPropagation()
        if (opts.allowed && !opts.allowed(e)) return
        count.current -= 1
        if (dropHover && count.current <= 0) setDropHover(false)
    }

    function onDragOver(e: DragEvent) {
        e.preventDefault()
    }
    
    const _onDrop = useCallback((e: DragEvent) => {
        e.preventDefault()
        e.stopPropagation()
        count.current = 0
        setDropHover(false)
        opts.onDrop?.(e)
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [opts.onDrop, ...(deps || [])])

    return {
      dropHover,
      dropTargetProps: {
        onDragEnter,
        onDragLeave,
        onDragOver,
        onDrop: opts.onDrop ? _onDrop : undefined,
      },
    }
}
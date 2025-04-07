import { useRef } from "react"

export default function useDialog(opts?: { modal?: boolean }) {
    const ref = useRef<HTMLDialogElement>(null)
    
    function show() {
        if (!ref.current) return
            
        if (opts?.modal) {
            ref.current.showModal()
        } else {
            ref.current.show()
        }
    }
    
    function close() {
        if (!ref.current) return
        ref.current.close()
    }
    
    function toggle() {
        if (!ref.current) return

        if (ref.current.hasAttribute("open")) close()
        else show()
    }

    function onClick(e: React.MouseEvent) {
        if (e.target !== ref.current) return

        const { clientX, clientY } = e
        const rect = e.currentTarget.getBoundingClientRect()
        const outside =
            clientX < rect.left ||
            clientX > rect.right ||
            clientY < rect.top ||
            clientY > rect.bottom
        if (outside) close()
    }

    return { dialogProps: { ref, onClick }, show, close, toggle }
}
import { createContext } from "react"

export interface ModalContext {
    close: () => void
}

export const ModalContext = createContext<ModalContext | null>(null)
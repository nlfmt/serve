import { createContext } from "react";

export interface ToastContext {
    remove: () => void
}

export const ToastContext = createContext<ToastContext | null>(null)
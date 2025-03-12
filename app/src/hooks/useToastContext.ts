import { ToastContext } from "@/contexts/Toast.context";
import { useContext } from "react";

export default function useToastContext() {
    const toastContext = useContext(ToastContext)
    if (!toastContext) throw new Error("ToastContext was not provided")
    return toastContext
}
import { ToastServiceContext } from "@/contexts/ToastService.context";
import { useContext } from "react";

export default function useToastService() {
    const toastService = useContext(ToastServiceContext)
    if (!toastService) throw new Error("ToastServiceContext was not provided")
    return toastService
}
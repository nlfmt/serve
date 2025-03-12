import { AppContext } from "@/contexts/App.context";
import { useContext } from "react";

export default function useAppContext() {
    const appContext = useContext(AppContext)
    if (!appContext) throw new Error()
    return appContext
}
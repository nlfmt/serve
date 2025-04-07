import { Settings } from "@/services/api"
import { createContext } from "react"

export const SettingsContext = createContext<Settings | null>(null)

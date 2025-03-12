import { createContext } from "react"
import { Navigation } from "./providers/Navigation.provider"

export const NavigationContext = createContext<Navigation | null>(null)

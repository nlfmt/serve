import { SettingsContext } from "@/contexts/Settings.context"
import { useContext } from "react"

export function useSettings() {
  const settings = useContext(SettingsContext)
  if (!settings) throw new Error("SettingsContext was not provided")
  return settings
}
import { NavigationContext } from "@/contexts/Navigation.context"
import { useContext } from "react"

export function useNavigation() {
  const navigation = useContext(NavigationContext)
  if (!navigation) throw new Error("NavigationContext was not provided")
  return navigation
}

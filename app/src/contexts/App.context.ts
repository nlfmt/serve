import { createContext } from "react";

export const AppContext = createContext<{ reload: () => void } | null>(null)
import { StrictMode } from "react"
import { createRoot } from "react-dom/client"

import "@exuanbo/file-icons-js/dist/css/file-icons.css"

import "./index.scss"
import App from "./App.tsx"

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)

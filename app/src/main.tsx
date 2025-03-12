import { createRoot } from "react-dom/client"

import "@exuanbo/file-icons-js/dist/css/file-icons.css"
import "@/styles/icons.scss"

import "./index.scss"
import App from "./App.tsx"

createRoot(document.getElementById("root")!).render(
  <App />
)

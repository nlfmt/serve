import c from "./App.module.scss"
import DirView from "./components/DirView/DirView"
import Breadcrumbs from "./components/Breadcrumbs/Breadcrumbs"
import UploadDialog from "./components/UploadDialog/UploadDialog"
import ToastManager from "./components/ToastManager/ToastManager"
import { useEffect, useState } from "react"
import api from "./services/api"
import { NavigationProvider } from "./contexts/providers/Navigation.provider"

function App() {
  const [uploadEnabled, setUploadEnabled] = useState(false)

  useEffect(() => {
    api.isUploadEnabled().then(res => {
      if (res.value) setUploadEnabled(true)
    })
  }, [])

  return (
    <NavigationProvider>
      <ToastManager>
        <div className={c.container}>
          <header className={c.header}>
            <h2 className={c.title}>File Explorer</h2>
            {uploadEnabled && <UploadDialog />}
          </header>
          <Breadcrumbs />
          <DirView />
        </div>
      </ToastManager>
    </NavigationProvider>
  )
}

export default App
function setLoading(arg0: boolean) {
  throw new Error("Function not implemented.")
}


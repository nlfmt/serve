import c from "./App.module.scss"
import DirView from "./components/DirView/DirView"
import Breadcrumbs from "./components/Breadcrumbs/Breadcrumbs"
import UploadDialog from "./components/UploadDialog/UploadDialog"
import ToastManager from "./components/ToastManager/ToastManager"
import { useEffect, useState } from "react"
import api, { Settings } from "./services/api"
import { NavigationProvider } from "./contexts/providers/Navigation.provider"
import QRCodeDialog from "./components/QRCodeDialog/QRCodeDialog"
import { SettingsContext } from "./contexts/Settings.context"
import ModalManager from "./components/ModalManager/ModalManager"


function App() {
  const [settings, setSettings] = useState<Settings>({
    allow_delete: false,
    allow_rename: false,
    upload: false,
  })

  useEffect(() => {
    api.getSettings().then(res => {
      if (res.value) setSettings(res.value)
    })
  }, [])

  return (
    <SettingsContext.Provider value={settings}>
      <NavigationProvider>
        <ToastManager>
          <ModalManager>
            <div className={c.container}>
              <header className={c.header}>
                <h2 className={c.title}><span>serve</span><span className={c.version}>v0.3.1</span></h2>
                <QRCodeDialog />
                {settings?.upload && <UploadDialog />}
              </header>
              <Breadcrumbs />
              <DirView />
            </div>
          </ModalManager>
        </ToastManager>
      </NavigationProvider>
    </SettingsContext.Provider>
  )
}

export default App

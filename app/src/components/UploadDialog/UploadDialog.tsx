import useDialog from "@/hooks/useDialog"
import Button from "../Button/Button"
import Dialog from "../Dialog/Dialog"
import c from "./UploadDialog.module.scss"
import { BackupOutlined, DeleteOutline } from "@mui/icons-material"
import { useCallback, useEffect, useRef, useState } from "react"
import useDropTarget from "@/hooks/useDropTarget"
import { sizeString } from "@/util/util"
import FileIcon from "../FileIcon/FileIcon"
import TextField from "../TextField/TextField"
import { useNavigation } from "@/hooks/useNavigation"
import useToastService from "@/hooks/useToastService"
import UploadToast from "../UploadToast/UploadToast"


function UploadDialog() {
  const { dialogProps, show, close } = useDialog({ modal: true })
  const toastService = useToastService()

  const fileInputRef = useRef<HTMLInputElement>(null)

  const [file, setFile] = useState<File | null>(null)
  const { dropHover, dropTargetProps } = useDropTarget(e => {
    const file = e.dataTransfer.files[0]
    setFile(file)
  })

  const { path } = useNavigation()
  const [uploadPath, setUploadPath] = useState(path)
  
  useEffect(() => {
    setUploadPath(path)
  }, [path])


  const upload = useCallback(() => {
    if (!file) return
      toastService.add(UploadToast, { file, path })
    close()
  }, [path, file, toastService, close])

  return (
    <>
      <Button onClick={show}>Upload</Button>
      <Dialog {...dialogProps}>
        <form className={c.container}>
          <header className={c.header}>
            <BackupOutlined className={c.icon} />
            <span>Upload a File</span>
          </header>
          <div className={c.content}>
            {!file ? (
              <>
                <main
                  data-drophover={dropHover}
                  className={c.dropZone}
                  {...dropTargetProps}
                  onClick={() => fileInputRef.current?.click()}
                >
                  <span>
                    Drag a file here,
                    <br />
                    or click to select one
                  </span>
                </main>
                <input
                  ref={fileInputRef}
                  type="file"
                  style={{ display: "none" }}
                  onChange={e => {
                    const file = e.target.files?.[0]
                    if (file) setFile(file)
                  }}
                />
              </>
            ) : (
              <>
                <main className={c.file}>
                  <FileIcon file={file.name} />
                  <div className={c.fileInfo}>
                    <span title={file.name}>{file.name}</span>
                    <span className={c.fileSize}>
                      Size: {sizeString(file.size)}
                    </span>
                  </div>
                  <div className={c.clearButton} onClick={() => setFile(null)}>
                    <DeleteOutline />
                  </div>
                </main>
                <div className={c.path}>
                  <span>Upload Path</span>
                  <TextField
                    className={c.pathInput}
                    placeholder="File Path"
                    autoFocus={true}
                    value={uploadPath}
                    onChange={e => setUploadPath(e.target.value)}
                  />
                </div>
              </>
            )}
          </div>
          <footer className={c.buttons}>
            <Button
              type="button"
              variant="secondary"
              onClick={() => {
                setFile(null)
                close()
              }}
            >
              Cancel
            </Button>
            {file && <Button onClick={upload}>Upload</Button>}
          </footer>
        </form>
      </Dialog>
    </>
  )
}

export default UploadDialog

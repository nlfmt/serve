import useToastContext from "@/hooks/useToastContext"
import api from "@/services/api"
import { AxiosProgressEvent } from "axios"
import { useCallback, useEffect, useRef, useState } from "react"
import c from "./UploadToast.module.scss"
import { durationString, sizeString } from "@/util/util"
import { classes } from "@/util/classes"
import Button from "../Button/Button"
import { useSettings } from "@/hooks/useSettings"
import { CancelOutlined } from "@mui/icons-material"

namespace UploadToast {
  export interface Props {
    path: string
    file: File
    onSuccess?: () => void
  }
}

function UploadToast({ file, path, onSuccess }: UploadToast.Props) {
  const { remove } = useToastContext()
  const { overwrite } = useSettings()
  const [progress, setProgress] = useState<AxiosProgressEvent | null>(null)
  const [error, setError] = useState<{ code: number; text: string } | null>(
    null,
  )
  const [success, setSuccess] = useState(false)
  const ctrl = useRef(new AbortController())

  const upload = useCallback((overwrite = false) => {
    api
      .uploadFile(
        path,
        file,
        overwrite,
        ctrl.current.signal,
        setProgress,
      )
      .then(res => {
        if (res.ok) {
          setSuccess(true)
          onSuccess?.()
          setTimeout(remove, 8000)
        } else {
          setError(res.error)
        }
      })
  }, [file, path, onSuccess, remove])

  useEffect(() => {
    upload(false)
  }, [upload])

  function abort() {
    ctrl.current.abort()
    setTimeout(remove, 5000)
  }

  return (
    <div className={c.container}>
      <header className={c.header}>
        <span className={c.title}>
          {error ? "" : "Uploading "}
          <span title={file.name} className={classes([c.green, !error])}>
            {file.name}
          </span>
        </span>
        {progress && !error && !success && (
          <div onClick={abort} className={c.abort}>
            <CancelOutlined />
          </div>
        )}
      </header>
      {!(error || success) && (
        <>
          {progress && (
            <span className={c.stats}>
              <span>
                {sizeString(progress?.loaded)} /{" "}
                {progress.total ? sizeString(file.size) : "?? B"}
              </span>
              <span>{sizeString(progress.rate ?? 0)}/s, {durationString(progress.estimated ?? 0)} left</span>
            </span>
          )}
          <div className={c.progress}>
            <div
              className={c.progressValue}
              style={{ width: `${100 * (progress?.progress ?? 0)}%` }}
            ></div>
          </div>
        </>
      )}
      {error && (
        <div className={c.message}>
          <span className={c.red}>{error.text}</span>
          <Button
            size="small"
            variant="secondary"
            onClick={() => remove()}
            style={{ marginLeft: "auto" }}
          >
            Cancel
          </Button>
          {error.code === 409 && overwrite ? (
            <Button
              size="small"
              onClick={() => {
                setError(null)
                upload(true)
              }}
            >
              Overwrite
            </Button>
          ) : (
            <Button
              size="small"
              onClick={() => {
                setError(null)
                upload()
              }}
            >
              Retry
            </Button>
          )}
        </div>
      )}
      {success && (
        <div className={c.message}>
          <span className={c.green}>Success</span>
        </div>
      )}
    </div>
  )
}

export default UploadToast

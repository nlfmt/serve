import useToastContext from "@/hooks/useToastContext"
import api from "@/services/api"
import { AxiosProgressEvent } from "axios"
import { useCallback, useEffect, useRef, useState } from "react"
import c from "./UploadToast.module.scss"
import { durationString, sizeString } from "@/util/util"
import { classes } from "@/util/classes"
import Button from "../Button/Button"

namespace UploadToast {
  export interface Props {
    path: string
    file: File
    onSuccess?: () => void
  }
}

function UploadToast(props: UploadToast.Props) {
  const { remove } = useToastContext()
  const [progress, setProgress] = useState<AxiosProgressEvent | null>(null)
  const [error, setError] = useState<{ code: number; text: string } | null>(
    null,
  )
  const [success, setSuccess] = useState(false)
  const ctrl = useRef(new AbortController())

  const upload = useCallback((overwrite = false) => {
    api
      .uploadFile(
        props.path,
        props.file,
        overwrite,
        ctrl.current.signal,
        setProgress,
      )
      .then(res => {
        if (res.ok) {
          setSuccess(true)
          props.onSuccess?.()
          setTimeout(remove, 3000)
        } else {
          setError(res.error)
        }
      })
  }, [])

  useEffect(() => {
    upload(false)
  }, [upload])

  function _abort() {
    ctrl.current.abort()
    setError({ code: 400, text: "Aborted" })
    setTimeout(remove, 3000)
  }

  return (
    <div className={c.container}>
      <header className={c.header}>
        <span className={c.title}>
          {error ? "" : "Uploading "}
          <span title={props.file.name} className={classes([c.green, !error])}>
            {props.file.name}
          </span>
        </span>
        {progress && !error && !success && (
          <span className={c.time}>
            {durationString(progress.estimated ?? 0)}
          </span>
        )}
      </header>
      {!(error || success) && (
        <>
          {progress && (
            <span className={c.stats}>
              <span>
                {sizeString(progress?.loaded)} /{" "}
                {progress.total ? sizeString(props.file.size) : "?? B"}
              </span>
              <span>{sizeString(progress.rate ?? 0)}/s</span>
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
          {error.code === 409 ? (
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

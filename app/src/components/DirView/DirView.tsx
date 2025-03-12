import { classes } from "@/util/classes"
import c from "./DirView.module.scss"
import useDropTarget from "@/hooks/useDropTarget"
import { ArrowBackRounded, ArrowForwardRounded, ArrowUpwardRounded, CachedRounded, FolderOutlined, ReplayRounded, SearchOutlined } from "@mui/icons-material"
import { DragEvent, useCallback, useEffect, useMemo, useState } from "react"
import { DirInfo } from "@/util/models"
import DirEntry from "../DirEntry/DirEntry"
import FileIcon from "../FileIcon/FileIcon"
import { dirInfo, joinPath } from "@/util/util"
import { useNavigation } from "@/hooks/useNavigation"
import api from "@/services/api"
import common from "../../styles/common.module.scss"
import TextField from "../TextField/TextField"
import useToastService from "@/hooks/useToastService"
import UploadToast from "../UploadToast/UploadToast"


function DirView() {
  const { path, navigate, up } = useNavigation()
  const toastService = useToastService()
  const [search, setSearch] = useState("")
  const [dirData, setDirData] = useState<DirInfo | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState("")
  
  const fetchFiles = useCallback(() => {
    setLoading(true)
    api.fetchFiles(path).then(res => {
      if (res.ok) setDirData(res.value)
      else setError(res.error)
      setLoading(false)
    })
  }, [path])

  useEffect(() => {
    fetchFiles()
  }, [fetchFiles])

  const data = useMemo(() => {
    if (!dirData) return null

    const searchFn = ({ name }: { name: string }) => {
      return name.toLowerCase().includes(search.toLowerCase())
    }

    return {
      files: dirData.files.filter(searchFn),
      dirs: dirData.dirs.filter(searchFn),
    }
  }, [search, dirData])
  
  const { dropHover, dropTargetProps } = useDropTarget(async (e: DragEvent) => {
    const file = e.dataTransfer.files[0]
    if (!file) return
    toastService.add(UploadToast, { file, path, onSuccess: fetchFiles })
  }, [path])

  return (
    <div
      className={classes(c.fileView, [c.dragover, dropHover])}
      {...dropTargetProps}
    >
      <header className={c.toolbar}>
        <div className={c.navigation}>
          <div
            title="Up one level"
            className={c.navigationButton}
            onClick={() => {
              if (path !== "/") up()
            }}
          >
            <ArrowUpwardRounded />
          </div>
          <div
            title="Previous folder"
            className={c.navigationButton}
            onClick={() => window.history.go(-1)}
          >
            <ArrowBackRounded />
          </div>
          <div
            title="Next folder"
            className={c.navigationButton}
            onClick={() => window.history.go(1)}
          >
            <ArrowForwardRounded />
          </div>
          <div
            title="Reload Folder"
            className={classes(c.navigationButton, [c.rotate, loading])}
            onClick={() => {
              if (!loading) fetchFiles()
            }}
          >
            <ReplayRounded />
          </div>
        </div>
        {loading && <span className={c.loading}>Loading...</span>}
        <TextField className={c.searchBar} icon={<SearchOutlined />} value={search} onChange={e => setSearch(e.target.value)} />
      </header>
      <section className={c.labels}>
        <span></span>
        <span>Name</span>
        <span>Size</span>
        <span className={common.hideOnMobile}>Modified</span>
        <span className={common.hideOnTablet}>Created</span>
      </section>
      <main className={c.content}>
        {data ? (
          <>
            {data.dirs.map(info => (
              <DirEntry
                key={info.name}
                info={info}
                icon={<FolderOutlined />}
                onClick={() => navigate(info.name)}
                onDrop={e => {
                  const file = e.dataTransfer.files[0]
                  if (!file) return
                  toastService.add(UploadToast, {
                    file,
                    path: joinPath(path, info.name),
                    onSuccess: fetchFiles,
                  })
                }}
              />
            ))}
            {data.files.map(info => {
              const params = new URLSearchParams({
                path: joinPath(path, info.name),
              })
              return (
                <DirEntry
                  key={info.name}
                  info={info}
                  icon={<FileIcon file={info.name} />}
                  download={`/api/download?${params.toString()}`}
                />
              )
            })}
            {data.dirs.length + data.files.length == 0 &&
              (search ? (
                <span className={c.placeholder}>
                  No Files found for '{search}'
                </span>
              ) : (
                <span className={c.placeholder}>No files</span>
              ))}
          </>
        ) : (
          <span className={c.placeholder}>{error || "Loading..."}</span>
        )}
        {loading && (
          <div className={c.loadingSpinner}>
            <CachedRounded />
          </div>
        )}
      </main>
      <footer className={c.footer}>
        <span>{data && dirInfo(data)}</span>
        <span>serve v0.3.0</span>
      </footer>
    </div>
  )
}

export default DirView
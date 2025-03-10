import { useCallback, useEffect, useMemo, useState } from "react"
import c from "./App.module.scss"
import {
  ArrowBackRounded,
  ArrowForwardRounded,
  ArrowUpwardRounded,
  CachedRounded,
  FolderOutlined,
  HomeRounded,
  ReplayRounded,
  SearchOutlined,
} from "@mui/icons-material"
import { DirInfo } from "@/util/models"
import { dirInfo } from "@/util/util"
import DirEntry from "@/components/DirEntry/DirEntry"
import FileIcon from "@/components/FileIcon/FileIcon"
import useNavigation from "@/hooks/useNavigation"
import { classes } from "./util/classes"

const API_URL = `http://${location.host}/api`

function App() {
  const { path, navigate, up } = useNavigation(location.pathname)

  const [search, setSearch] = useState("")
  const [dirData, setDirData] = useState<DirInfo | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState("")
  
  const fetchFiles = useCallback(() => {
    setLoading(true)

    ;(async () => {
      const query = new URLSearchParams({ path: "." + path })
      const res = await fetch(API_URL + `/files?${query.toString()}`, {})
      if (!res.ok) {
        console.log(res)
        setError(`Error: ${await res.text()}`)
        return
      }
      const data = await res.json()
      setDirData(data)
    })().then(() => {
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

  const [dragOver, setDragOver] = useState(false);
  return (
    <div className={c.container}>
      <h2 className={c.title}>File Explorer</h2>
      {/* <header className={c.header}> */}
        <Breadcrumbs path={path} />
        {/* <Button variant="secondary" className={c.zipButton}>
          ZIP
        </Button> */}
      {/* </header> */}
      <div className={classes(c.fileView, [c.dragover, dragOver])} onDragOver={(e) => setDragOver(true)} onDragLeave={() => setDragOver(false)}>
        <header className={c.toolbar}>
          <div className={c.navigation}>
            <div title="Up one level" className={c.navigationButton} onClick={() => {
              if (path === "/") return
              up()
            }}>
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
              onClick={() => { if (!loading) fetchFiles() }}
            >
              <ReplayRounded />
            </div>
          </div>
          {loading && <span className={c.loading}>Loading...</span>}
          <div className={c.searchBar}>
            <SearchOutlined />
            <input
              type="text"
              placeholder="Search"
              value={search}
              onChange={e => setSearch(e.target.value)}
            />
          </div>
        </header>
        <section className={c.labels}>
          <span></span>
          <span>Name</span>
          <span>Size</span>
          <span className={c.hideOnMobile}>Modified</span>
          <span className={c.hideOnTablet}>Created</span>
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
                />
              ))}
              {data.files.map(info => {
                const params = new URLSearchParams({
                  path:
                    path === "/"
                      ? "./" + info.name
                      : "." + path + "/" + info.name,
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
          {loading && <div className={c.loadingSpinner}><CachedRounded /></div>}
        </main>
        <footer className={c.footer}>
          <span>{data && dirInfo(data)}</span>
          <span>serve v0.3.0</span>
        </footer>
      </div>
    </div>
  )
}

export default App

function Breadcrumbs(props: { path: string }) {
  const segments = useMemo(() => {
    const segments = props.path.match(/\/|([^/]+)/g)
    return segments ? [...segments] : []
  }, [props.path])

  return (
    <div className={c.breadcrumbs}>
      <div
        className={c.homeIcon}
        onClick={() => window.history.pushState(null, "", "/")}
      >
        <HomeRounded sx={{ fontSize: 20 }} />
      </div>
      <div className={c.path}>
        {segments.map((s, i) => (
          <span
            key={i}
            className={s === "/" ? c.pathSlash : c.pathSegment}
            onClick={
              s !== "/"
                ? () => {
                    window.history.pushState(
                      null,
                      "",
                      segments.slice(0, i + 1).join(""),
                    )
                  }
                : undefined
            }
          >
            {s}
          </span>
        ))}
      </div>
    </div>
  )
}

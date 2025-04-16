import { useMemo } from "react"
import c from "./Breadcrumbs.module.scss"
import { CreateNewFolderOutlined, FolderZipOutlined, HomeRounded, } from "@mui/icons-material"
import { useNavigation } from "@/hooks/useNavigation"
import ContextMenu from "../ContextMenu/ContextMenu"
import api from "@/services/api"
import useToastService from "@/hooks/useToastService"
import useModalService from "@/hooks/useModalService"
import TextInputModal from "../TextInputModal/TextInputModal"
import Toast from "../Toast/Toast"
import { joinPath } from "@/util/util"
import useDropTarget from "@/hooks/useDropTarget"
import UploadToast from "../UploadToast/UploadToast"
import useFileActions from "@/hooks/useFileActions"

function Breadcrumbs(props: { reload: () => void }) {
  const { path, home } = useNavigation()
  const toastService = useToastService()
  const modalService = useModalService()

  const segments = useMemo(() => {
    const segments = path.match(/\/|([^/]+)/g)
    return segments ? [...segments] : []
  }, [path])
  
  async function createFolder() {
    modalService.show(TextInputModal, {
      title: "Create Folder",
      confirmText: "Create",
      placeholder: "Folder Name",
      onConfirm: value => {
        api.createFolder(joinPath(path, value)).then(() => {
          toastService.add(Toast, { text: `Created '${value}'`, type: "success" })
          props.reload()
        }).catch(e => {
          toastService.add(Toast, { text: `Error creating folder: ${e}`, type: "error" })
        })
      }
    })
  }

  const { move } = useFileActions({ reload: props.reload })
  
  const { dropHover, dropTargetProps } = useDropTarget({
    onDrop: e => {
      const file = e.dataTransfer.files[0]
      const srcPath = e.dataTransfer.getData("path")
      if (file) {
        toastService.add(UploadToast, {
          file,
          path: "/"
        })
      } else if (srcPath) {
        move(srcPath, "/")
      }
    },
  })

  return (
    <div className={c.breadcrumbs}>
      <div data-drophover={dropHover} className={c.homeIcon} onClick={home} {...dropTargetProps}>
        <HomeRounded sx={{ fontSize: 20 }} />
      </div>

      <div className={c.path}>
        {segments.map((s, i) =>
          s === "/" ? (
            <span key={i} className={c.pathSlash}>
              {s}
            </span>
          ) : (
            <Segment
              key={i}
              reload={props.reload}
              move={move}
              path={segments.slice(0, i + 1).join("")}
              onClick={() => {
                window.history.pushState(
                  null,
                  "",
                  segments.slice(0, i + 1).join(""),
                )
              }}
            >
              {s}
            </Segment>
          ),
        )}
      </div>

      <ContextMenu className={c.contextMenu}>
        <ContextMenu.Item
          label="Download ZIP"
          icon={<FolderZipOutlined />}
          onClick={() => api.downloadFolder(path)}
        />
        <ContextMenu.Item
          label="Create Folder"
          icon={<CreateNewFolderOutlined />}
          onClick={createFolder}
        />
      </ContextMenu>
    </div>
  )
}

function Segment(props: {
  children: string
  path: string
  onClick: () => void
  reload: () => void
  move: (path: string, dest: string) => void
}) {
  const toastService = useToastService()

  const { dropHover, dropTargetProps } = useDropTarget({
    onDrop: e => {
      const file = e.dataTransfer.files[0]
      const srcPath = e.dataTransfer.getData("path")
      if (file) {
        toastService.add(UploadToast, {
          file,
          path: props.path,
        })
      } else if (srcPath) {
        props.move(srcPath, props.path)
      }
    },
  })

  return (
    <span
      data-drophover={dropHover}
      className={c.pathSegment}
      onClick={props.onClick}
      {...dropTargetProps}
    >
      {props.children}
    </span>
  )
}

export default Breadcrumbs

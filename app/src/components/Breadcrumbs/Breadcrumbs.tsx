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

  return (
    <div className={c.breadcrumbs}>
      <div
        className={c.homeIcon}
        onClick={home}
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
      
      {/* <a download target="_blank" href={downloadUrl} className={c.downloadButton}>
      <Button variant="primary">Download Folder</Button>
      </a> */}
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

export default Breadcrumbs

import api from "@/services/api"
import useModalService from "./useModalService"
import useToastService from "./useToastService"
import Toast from "@/components/Toast/Toast"
import TextInputModal from "@/components/TextInputModal/TextInputModal"
import ConfirmModal from "@/components/ConfirmModal/ConfirmModal"

export default function useFileActions({ reload }: { reload: () => void }) {
    const toastService = useToastService()
    const modalService = useModalService()

  async function remove(path: string) {
    modalService.show(ConfirmModal, {
      title: `Delete '${path}'`,
      text: "Are you sure? This action can't be undone",
      confirmText: "Delete",
      onConfirm: async () => {
        const res = await api.remove(path)

        if (res.error) {
          toastService.add(Toast, {
            text: `Failed to delete '${path}': ${res.error}`,
            type: "error",
          })
        } else {
          toastService.add(Toast, {
            text: `'${path}' was deleted`,
            type: "success",
          })
          reload()
        }
      }
    })
  }
  
  async function rename(path: string) {
    modalService.show(TextInputModal, {
      title: `Rename ${path}`,
      confirmText: "Rename",
      placeholder: "Enter new name..",
      initialValue: path.split("/").at(-1),
      onConfirm: async value => {
        const res = await api.rename(path, value)
        if (res.ok) {
          toastService.add(Toast, {
            text: `Renamed '${path}' to '${value}'`,
            type: "success",
          })
          reload()
        } else {
          toastService.add(Toast, {
            text: `Failed to rename '${path}': ${res.error}`,
            type: "error"
          })
        }
      },
    })
  }
  
  async function move(path: string, dest: string) {
    const res = await api.move(path, dest)

    if (res.error) {
      toastService.add(Toast, { text: `Failed to move '${path}': ${res.error}`, type: "error" })
    } else {
      toastService.add(Toast, { text: `Moved '${path}' to '${dest}'`, type: "success" })
      reload()
    }
  }
  
  return { rename, remove, move }
}

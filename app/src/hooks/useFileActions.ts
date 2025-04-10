import api from "@/services/api"
import useModalService from "./useModalService"
import useToastService from "./useToastService"
import Toast from "@/components/Toast/Toast"
import TextInputModal from "@/components/TextInputModal/TextInputModal"

export default function useFileActions({ reload }: { reload: () => void }) {
    const toastService = useToastService()
    const modalService = useModalService()

  async function remove(path: string) {
    const res = await api.remove(path)
    if (res.error) {
      toastService.add(Toast, { text: `Failed to delete '${path}': ${res.error}`, type: "error" })
    } else {
      toastService.add(Toast, { text: `'${path}' was deleted`, type: "success" })
      reload()
    }
  }
  
  async function rename(path: string) {
    modalService.show(TextInputModal, {
      title: `Rename ${path}`,
      confirmText: "Rename",
      placeholder: "Enter new name..",
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
  
  return { rename, remove }
}
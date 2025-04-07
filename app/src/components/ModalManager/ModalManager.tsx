import useDialog from "@/hooks/useDialog"
import Dialog from "../Dialog/Dialog"
import { ReactNode, useState } from "react"
import { ModalServiceContext } from "@/contexts/ModalService.context"
import { ModalContext } from "@/contexts/Modal.context"


function ModalManager({ children }: { children: ReactNode }) {
  const { dialogProps, show: showDialog, close: closeDialog } = useDialog({ modal: true })
  
  const [modal, setModal] = useState<{
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    Modal: React.ComponentType<any>,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    props: any,
  } | null>(null)
  
  function show<T>(modal: React.ComponentType<T>, props: T) {
    setModal({
      Modal: modal,
      props,
    })
    showDialog()
  }
  
  function close() {
    setModal(null)
    closeDialog()
  }

  return (
    <>
      <ModalServiceContext.Provider value={{ show }}>
        {children}
      </ModalServiceContext.Provider>

      <ModalContext.Provider value={{ close }}>
        <Dialog {...dialogProps}>
          {modal && <modal.Modal {...modal.props} />}
        </Dialog>
      </ModalContext.Provider>
    </>
  )
}

export default ModalManager

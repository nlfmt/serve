import useModalContext from "@/hooks/useModalContext";
import Button from "../Button/Button";
import c from "./ConfirmModal.module.scss"

namespace ConfirmModal {
  export interface Props {
    title: string
    text: string
    confirmText: string
    onConfirm?: () => void
  }
}

function ConfirmModal(props: ConfirmModal.Props) {
  const { close } = useModalContext()
  
  return (
    <form className={c.container}>
      <span>{props.title}</span>
      <p>{props.text}</p>

      <div className={c.buttons}>
        <Button type="button" variant="secondary" onClick={close}>
          Cancel
        </Button>
        <Button onClick={() => {
          close()
          props.onConfirm?.()
        }}>
          {props.confirmText}
        </Button>
      </div>
    </form>
  )
}

export default ConfirmModal
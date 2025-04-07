import useDialog from "@/hooks/useDialog";
import Button from "../Button/Button";
import c from "./QRCodeDialog.module.scss"
import { QrCodeOutlined } from "@mui/icons-material";

function QRCodeDialog() {
  const { toggle, dialogProps } = useDialog({ modal: true })

  return (
    <>
      <Button className={c.button} variant="secondary" onClick={toggle}>
        <QrCodeOutlined />
      </Button>
      <dialog {...dialogProps} className={c.dialog}>
        <img src="/api/qr" alt="QR Code" onClick={toggle} />
      </dialog>
    </>
  )
}

export default QRCodeDialog
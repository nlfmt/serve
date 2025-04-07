import useModalContext from "@/hooks/useModalContext";
import Button from "../Button/Button";
import TextField from "../TextField/TextField";
import { useState } from "react";
import c from "./RenameModal.module.scss"
import api from "@/services/api";
import useToastService from "@/hooks/useToastService";
import ErrorToast from "../ErrorToast/ErrorToast";

function RenameModal(props: { path: string, onSuccess?: () => void }) {
    const { close } = useModalContext()
    const toastService = useToastService()
    
    const [name, setName] = useState("")
    
    async function rename() {
        close()
        const res = await api.rename(props.path, name)
        if (res.ok) {
          toastService.add(ErrorToast, { error: `Renamed ${props.path} to ${name}` })
          props.onSuccess?.()
        } else {
          toastService.add(ErrorToast, { error: `Failed to rename ${props.path}: ${res.error}` })
        }

    }

    return (
      <form className={c.container}>
        <span>Rename {props.path}</span>
        <TextField
          className={c.input}
          value={name}
          onChange={e => setName(e.target.value)}
          placeholder="Enter new name..."
        />

        <div className={c.buttons}>
          <Button type="button" variant="secondary" onClick={close}>Cancel</Button>
          <Button disabled={!name} onClick={rename}>Rename</Button>
        </div>
      </form>
    )
}

export default RenameModal
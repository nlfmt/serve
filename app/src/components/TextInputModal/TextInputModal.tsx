import useModalContext from "@/hooks/useModalContext";
import Button from "../Button/Button";
import TextField from "../TextField/TextField";
import { useEffect, useRef, useState } from "react";
import c from "./TextInputModal.module.scss"

namespace TextInputModal {
  export interface Props {
    title: string
    placeholder?: string
    confirmText: string
    onConfirm?: (value: string) => void
    initialValue?: string
  }
}

function TextInputModal(props: TextInputModal.Props) {
  const { close } = useModalContext()
  const [value, setValue] = useState(props.initialValue ?? "")
  
  const inputRef = useRef<HTMLInputElement>(null)
  
  useEffect(() => {
    if (inputRef.current) {
      inputRef.current.select()
    }
  }, [])

  return (
    <form className={c.container}>
      <span>{props.title}</span>
      <TextField
        ref={inputRef}
        autoFocus={true}
        className={c.input}
        value={value}
        onChange={e => setValue(e.target.value)}
        placeholder={props.placeholder}
      />

      <div className={c.buttons}>
        <Button type="button" variant="secondary" onClick={close}>
          Cancel
        </Button>
        <Button disabled={!value} onClick={() => {
          close()
          props.onConfirm?.(value)
        }}>
          {props.confirmText}
        </Button>
      </div>
    </form>
  )
}

export default TextInputModal
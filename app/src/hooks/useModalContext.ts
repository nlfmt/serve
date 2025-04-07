import { ModalContext } from "@/contexts/Modal.context";
import { useContext } from "react";

export default function useModalContext() {
    const modalContext = useContext(ModalContext)
    if (!modalContext) throw new Error("ModalContext was not provided")
    return modalContext
}
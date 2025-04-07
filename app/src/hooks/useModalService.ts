import { ModalServiceContext } from "@/contexts/ModalService.context";
import { useContext } from "react";

export default function useModalService() {
    const modalService = useContext(ModalServiceContext)
    if (!modalService) throw new Error("ModalServiceContext was not provided")
    return modalService
}
import { createContext } from "react";

export interface ModalService {
    show<T>(modal: React.ComponentType<T>, props: T): void;
}

export const ModalServiceContext = createContext<ModalService | null>(null)
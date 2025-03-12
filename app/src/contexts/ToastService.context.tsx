import { createContext } from "react";

export interface ToastService {
    add<T>(toast: React.ComponentType<T>, props: T): void;
}

export const ToastServiceContext = createContext<ToastService | null>(null)
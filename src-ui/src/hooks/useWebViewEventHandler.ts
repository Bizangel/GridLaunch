import { useEffect } from "react"
import type { ToWebViewEvent} from "../ipc/common"

export const useWebViewEventHandler = <K extends ToWebViewEvent['type']>(type: K, callback: (ev: ToWebViewEvent) => void) => {
    useEffect(() => {
        const listener = (event: MessageEvent<any>) => {
            if (type == event.data.type) {
                callback(event.data)
            }
        }

        window.addEventListener("message", listener);
        return () => {
            window.removeEventListener("message", listener);
        }
    }, [callback, type])
}
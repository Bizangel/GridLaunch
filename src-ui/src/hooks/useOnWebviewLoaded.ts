import { useEffect } from "react";

export function useOnWebviewLoaded(callback: () => void) {
  useEffect(() => {
    if (typeof callback === "function") {
      callback();
    }
  }, [callback]);
}

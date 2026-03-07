export { };

declare global {
  interface Window {
    ipc: {
      postMessage: (string: string) => void;
    };
  }
}
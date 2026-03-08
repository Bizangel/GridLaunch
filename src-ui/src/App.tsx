import { useCallback, useState } from 'react'
import { sendIPCEvent } from './ipc/common'
import { useWebViewEventHandler } from './hooks/useWebViewEventHandler'

function App() {
  const [count, setCount] = useState(0)


  const callbutton = useCallback(() => {
    sendIPCEvent({type: 'LaunchRequested'})
  }, [])

  useWebViewEventHandler('AppGamepadButtonEvent', (ev) => {
    console.log("button pressed: ", ev)
  })

  return (
    <>
      <p>
        Not fancy UI button click should launch something

        <button onClick={callbutton}>
        press me
        </button>
      </p>
    </>
  )
}

export default App

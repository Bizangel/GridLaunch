import { useCallback, useState } from 'react'
import reactLogo from './assets/react.svg'
import './App.css'
import { sendIPCEvent } from './ipc/common'

type LaunchRequestedEvent = {
  type: 'FromWebViewEvent',
  event: {
    client: string
  }
}

// type FromWebViewEvents = LaunchRequestedEvent;

// const sendIPC(event: )


function App() {
  const [count, setCount] = useState(0)


  const callbutton = useCallback(() => {
    sendIPCEvent({type: 'LaunchRequested'})
  }, [])

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

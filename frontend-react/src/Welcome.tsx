import { useState } from "react";

interface Props {}

const Welcome: React.FC<Props> = (props) => {
  const [roomname, setRoomname] = useState("")
  const [username, setUsername] = useState("")

  function roomname_handler(event: React.ChangeEvent<HTMLInputElement>) {
    setRoomname(event.target.value)
    console.log(`setRoomname: ${event.target.value}`)
  }
  
  function username_handler(event: React.ChangeEvent<HTMLInputElement>) {
    setUsername(event.target.value)
    console.log(`setUsername: ${event.target.value}`)
  }

  function onClickHandler() {
    console.log(`emit join_handler(${roomname}, ${username})`)
  }

  return (
    // Special thanks https://tailwindui.com/preview#component-55b9c2097342175b8ddfccf8a30fb68f
    <div className="min-h-full flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md w-full space-y-8">
        <div>
          <h2 className="mt-6 text-center text-6xl font-extrabold text-gray-900">{"RustKnock"}</h2>
        </div>
        <form className="mt-8 space-y-6" action="#" method="POST">
          <input type="hidden" name="remember" value="true" />
          <div className="rounded-md shadow-sm -space-y-px">
            <div>
              <label htmlFor="roomname" className="sr-only">{"ルーム名"}</label>
              <input id="roomname" name="roomname" type="text" required={true} onChange={roomname_handler} className="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm" placeholder="ルーム名" />
            </div>
            <div>
              <label htmlFor="username" className="sr-only">{"ユーザー名"}</label>
              <input id="username" name="username" type="text" autoComplete="username" required={false} onChange={username_handler} className="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm" placeholder="ユーザー名" />
            </div>
          </div>

          <div>
            <button type="button" onClick={() => onClickHandler()} className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
              {"ルームに入る"}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
};

export default Welcome;

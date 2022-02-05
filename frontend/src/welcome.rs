use yew::html;
use yew::prelude::*;

#[function_component(Welcome)]
pub fn welcome() -> Html {
    html!(
      // Special thanks https://tailwindui.com/preview#component-55b9c2097342175b8ddfccf8a30fb68f
        <div class="min-h-full flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
          <div class="max-w-md w-full space-y-8">
            <div>
              // <img class="mx-auto h-12 w-auto" src="https://tailwindui.com/img/logos/workflow-mark-indigo-600.svg" alt="Workflow" />
              <h2 class="mt-6 text-center text-6xl font-extrabold text-gray-900">{"RustKnock"}</h2>
            </div>
            <form class="mt-8 space-y-6" action="#" method="POST">
              <input type="hidden" name="remember" value="true" />
              <div class="rounded-md shadow-sm -space-y-px">
                <div>
                  <label for="room-name" class="sr-only">{"ルーム名"}</label>
                  <input id="room-name" name="room-name" type="text" required=true class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm" placeholder="ルーム名" />
                </div>
                <div>
                  <label for="user-name" class="sr-only">{"ユーザー名"}</label>
                  <input id="user-name" name="user-name" type="text" autocomplete="username" required=false class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm" placeholder="ユーザー名" />
                </div>
              </div>

              <div>
                <button type="submit" class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                  {"ルームに入る"}
                </button>
              </div>
            </form>
          </div>
        </div>
    )
}

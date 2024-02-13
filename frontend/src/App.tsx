import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import BusyIndicator from "./Status";
import UserWidget, { UserContext, UserContextDispatch } from "./User";
import { useState } from "react";
function App() {
  const [user, setUser] = useState(null);
  return (
    <>
      <h1>Lunchtable</h1>
      <div>
        <UserContext.Provider value={user}>
          <UserContextDispatch.Provider value={setUser}>
            <UserWidget />
            <BusyIndicator />
          </UserContextDispatch.Provider>
        </UserContext.Provider>
      </div>
    </>
  );
}

export default App;

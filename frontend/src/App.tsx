import { BottomNavigation, BottomNavigationAction } from "@mui/material";
import "./App.css";
import BusyIndicator from "./Status";
import UserWidget, { UserContext, UserContextDispatch } from "./User";
import { useState } from "react";
import { MemoryRouter } from "react-router-dom";
import { StaticRouter } from "react-router-dom/server";

function Router(props: { children?: React.ReactNode }) {
  const { children } = props;
  if (typeof window === "undefined") {
    return <StaticRouter location="/">{children}</StaticRouter>;
  }

  return <MemoryRouter>{children}</MemoryRouter>;
}
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
            <Router>
              <BottomNavigation showLabels>
                <BottomNavigationAction label="Free/Busy"></BottomNavigationAction>
                <BottomNavigationAction label="Proposed Hangouts"></BottomNavigationAction>
                <BottomNavigationAction
                  label="About Me"
                  component={UserWidget}
                ></BottomNavigationAction>
              </BottomNavigation>
            </Router>
          </UserContextDispatch.Provider>
        </UserContext.Provider>
      </div>
    </>
  );
}

export default App;

import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import { useState } from "react";
import { gql, useQuery } from "@apollo/client";

const GET_STATUS_QUERY = gql(/* GraphQL */ `
  query GetStatus($user: Uuid!) {
    getStatus(user: $user) {
      kind
    }
  }
`);
const USER = "7f47f29f-0140-44f0-8d7f-b08779a09f8b";
function BusyIndicator() {
  const { loading, data } = useQuery(
    GET_STATUS_QUERY,

    // variables are also typed!

    { variables: { user: USER } },
  );
  const [status, setStatus] = useState("free");

  function toggleStatus() {
    console.log("Toggling");
    if (status === "free") {
      setStatus("busy");
    } else {
      setStatus("free");
    }
  }
  return (
    <div className="card">
      <h3>Status</h3>
      {loading ? (
        <p>Loading ...</p>
      ) : (
        <div>
          <button onClick={() => toggleStatus()}>
            status is
            {data &&
              data.kind.map((status: any) => {
                String(status);
              })}
          </button>
          <p>
            Edit <code>src/App.tsx</code> and save to test HMR
          </p>
        </div>
      )}
    </div>
  );
}
function App() {
  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <div>
        <BusyIndicator />
      </div>
      <h1>Lunchtable</h1>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  );
}

export default App;

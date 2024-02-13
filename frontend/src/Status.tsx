import { gql, useQuery } from "@apollo/client";
import { useContext, useState } from "react";
import { UserContext } from "./User";

const GET_STATUS_QUERY = gql(/* GraphQL */ `
  query GetStatus($user: Uuid!) {
    getStatus(user: $user) {
      kind
    }
  }
`);
function BusyIndicator() {
  const user = useContext(UserContext);
  const { loading, data } = useQuery(
    GET_STATUS_QUERY,

    // variables are also typed!

    { variables: { user: user } },
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
export default BusyIndicator;

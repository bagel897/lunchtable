import { gql, useMutation, useQuery } from "@apollo/client";
import { useContext } from "react";
import { UserContext } from "./User";

const GET_STATUS_QUERY = gql(/* GraphQL */ `
  query GetStatus($user: Uuid!) {
    getStatus(user: $user) {
      kind
    }
  }
`);
const SET_STATUS = gql(/* GraphQL */ `
  mutation SetStatus($user: Uuid!, $kind: StatusKind!) {
    setStatus(
      user: $user
      kind: $kind
      reason: MANUAL
      duration: "2016-01-01T13:10:20Z"
    ) {
      kind
    }
  }
`);
function BusyIndicator() {
  const user = useContext(UserContext);
  if (user == "" || user == null) {
    return "";
  }
  const { loading, data } = useQuery(
    GET_STATUS_QUERY,

    // variables are also typed!

    { variables: { user: user } },
  );
  const [mutateFunction] = useMutation(SET_STATUS);

  function toggleStatus(status: String) {
    console.log("Toggling");
    if (status === "free") {
      mutateFunction({ variables: { user: user, kind: "BUSY" } });
    } else {
      mutateFunction({ variables: { user: user, kind: "free" } });
    }
  }
  return (
    <div className="card">
      <h3>Status</h3>
      {loading ? (
        <p>Loading ...</p>
      ) : (
        <div>
          {console.log(data)}
          {data.kind &&
            data.kind.map((status: any) => {
              <button onClick={() => toggleStatus(status)}>
                status is {status}
              </button>;
            })}
        </div>
      )}
    </div>
  );
}
export default BusyIndicator;

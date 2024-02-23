import { useMutation, useQuery } from "@apollo/client";
import { useContext } from "react";
import { UserContext } from "./User";

import { gql } from "../src/__generated__/gql";
import { StatusKind } from "./__generated__/graphql";
import { Button } from "@mui/material";
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
  const [mutateFunction] = useMutation(SET_STATUS, {
    refetchQueries: [GET_STATUS_QUERY, "GetStatus"],
  });

  function toggleStatus(status: StatusKind) {
    if (status === StatusKind.Free) {
      mutateFunction({ variables: { user: user, kind: StatusKind.Busy } });
    } else {
      mutateFunction({ variables: { user: user, kind: StatusKind.Free } });
    }
  }
  return (
    <div className="card">
      {loading ? (
        <p>Loading ...</p>
      ) : (
        <div>
          {data && (
            <Button onClick={() => toggleStatus(data.getStatus.kind)}>
              {data.getStatus.kind}
            </Button>
          )}
        </div>
      )}
    </div>
  );
}
export default BusyIndicator;

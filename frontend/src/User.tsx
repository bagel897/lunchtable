import { gql, useMutation, useQuery } from "@apollo/client";
import { createContext, useContext } from "react";

const GET_USER_QUERY = gql(/* GraphQL */ `
  query GetUser($user: Uuid!) {
    getStatus(user: $user) {
      kind
    }
  }
`);
const CREATE_USER_MUTATION = gql(/* GraphQL */ `
  mutation ($name: String!) {
    createUser
  }
`);
export const UserContext = createContext("");
function UserName() {
  const user = useContext(UserContext);
  const { loading, data } = useQuery(
    GET_USER_QUERY,

    // variables are also typed!

    { variables: { user: user } },
  );
  return (
    <div>{loading ? <div> "loading" </div> : data && data.kind.map()} </div>
  );
}

function UserWidget() {
  const user = useContext(UserContext);
  const [mutateFunction, { data, loading, error }] = useMutation(
    CREATE_USER_MUTATION,
    { variables: { name: name } },
  );
  function createUser(name: String) {
    console.log("Creating User", name);
    mutateFunction({
      variables: { name: name },
      update(
        _,
        {
          data: {
            createUser: { id },
          },
        },
      ) {
        console.log("hi ", id);
        user = id;
      },
    });
  }
  return (
    <div className="card">
      {loading ? (
        "loading"
      ) : user != "" ? (
        <div>
          <UserName />
        </div>
      ) : (
        <div>
          <h3>User</h3>
          <input type="text" />
          <br />
          <button
            onClick={() => createUser(document.querySelector("input")?.value)}
          >
            Create User
            {/* {data && */}
            {/*   data.kind.map((status: any) => { */}
            {/*     String(status); */}
            {/*   })} */}
          </button>
          <p>
            Edit <code>src/App.tsx</code> and save to test HMR
          </p>
        </div>
      )}
    </div>
  );
}
export default UserWidget;

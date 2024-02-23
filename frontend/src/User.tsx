import { gql, useMutation, useQuery } from "@apollo/client";
import { createContext, useContext } from "react";
import Button from "@mui/material/Button";
import { TextField } from "@mui/material";
const GET_USER_QUERY = gql(/* GraphQL */ `
  query GetUser($user: Uuid!) {
    getUser(user: $user) {
      name
    }
  }
`);
const CREATE_USER_MUTATION = gql(/* GraphQL */ `
  mutation createUser($name: String!) {
    createUser(name: $name) {
      name
      id
    }
  }
`);
export const UserContext = createContext(null);
export const UserContextDispatch = createContext(null);
function UserName() {
  const user = useContext(UserContext);
  if (user == null) {
    return "";
  }
  const { loading, data } = useQuery(
    GET_USER_QUERY,

    // variables are also typed!

    { variables: { user: user } },
  );
  console.log("", data);
  return (
    <div>{loading ? <div> "loading" </div> : data && data.getUser.name} </div>
  );
}

function UserWidget() {
  var user = useContext(UserContext);
  var setUser = useContext(UserContextDispatch);
  const [mutateFunction] = useMutation(CREATE_USER_MUTATION);
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
        setUser(id);
      },
    });
  }
  return (
    <div className="card">
      {user != null ? (
        <div>
          <UserName />
        </div>
      ) : (
        <div>
          <TextField type="text" variant="outlined" />
          <br />
          <Button
            onClick={() => createUser(document.querySelector("input")?.value)}
          >
            Create User
            {/* {data && */}
            {/*   data.kind.map((status: any) => { */}
            {/*     String(status); */}
            {/*   })} */}
          </Button>
        </div>
      )}
    </div>
  );
}
export default UserWidget;

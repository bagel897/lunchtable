import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import React from "react";
class StatusState {
  state: String;
  constructor() {
    this.state = "free";
  }
}
class Status extends React.Component {
  state: StatusState;
  constructor(props: any) {
    super(props);
    this.state = new StatusState();
  }
  toggle() {
    console.log("Toggling");
    this.setState((state, props) => {
      if (state.state == "free") {
        this.state.state = "busy";
      } else {
        this.state.state = "free";
      }
    });
  }
  print() {
    return this.state.state;
  }
  render() {
    return (
      <button onClick={() => this.toggle()}>status is {this.print()}</button>
    );
  }
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
      <h1>Lunchtable</h1>
      <div className="card">
        <Status />
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  );
}

export default App;

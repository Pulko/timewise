import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

enum State {
  TODO = "to-do",
  IN_PROGRESS = "in-progress",
  DONE = "done",
}

enum StateLabel {
  "to-do" = "To Do",
  "in-progress" = "In Progress",
  "done" = "Done",
}

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [state, setState] = useState<State>(
    State.TODO
  );

  async function greet() {
    setGreetMsg(
      await invoke("greet", { name, state })
    );
  }

  async function clear() {
    await invoke("clear");
    setGreetMsg("Data is cleared out!");
  }

  async function remove() {
    await invoke("remove");
    setGreetMsg("Database is removed!");
  }

  return (
    <div className="container">
      <h1>Welcome to TimeWise!</h1>

      <p>
        Click on the Tauri, Vite, and React logos
        to learn more.
      </p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) =>
            setName(e.currentTarget.value)
          }
          placeholder="Enter a name..."
        />
        <select
          name="pets"
          id="pet-select"
          onChange={(e) => {
            console.log(e);
            setState(
              e.currentTarget.value as State
            );
          }}
        >
          {Object.values(State).map((state) => (
            <option value={state} key={state}>
              {StateLabel[state]}
            </option>
          ))}
        </select>
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>

      <button onClick={clear}>Clear</button>
      <button onClick={remove}>
        Remove Database
      </button>
    </div>
  );
}

export default App;

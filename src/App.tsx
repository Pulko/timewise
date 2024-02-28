import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNotification } from "./context/notifications";

interface Item {
  title: string;
  state: State;
}

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
  const [name, setName] = useState("");
  const [state, setState] = useState<State>(
    State.TODO
  );
  const [items, setItems] = useState<Item[]>([]);

  const { addNotification } = useNotification();

  useEffect(() => {
    fetch().then((data) => setItems(data));
  }, []);

  async function fetch(): Promise<Item[]> {
    const response = await invoke<string>(
      "fetch"
    );
    return JSON.parse(response);
  }

  async function add() {
    await invoke("add", { name, state })
      .then(() => fetch())
      .then((data) => setItems(data))
      .finally(() => {
        setName("");
        setState(State.TODO);
        addNotification("Item added!", 3000);
      });
  }

  async function clear() {
    await invoke("clear")
      .then(() => fetch())
      .then((data) => setItems(data))
      .finally(() => {
        setName("");
        setState(State.TODO);
        addNotification(
          "Data is cleared out!",
          3000
        );
      });
  }

  async function remove() {
    await invoke("remove").then(() => {
      setItems([]);
      addNotification(
        "Database is removed!",
        3000
      );
    });
  }

  return (
    <div className="container">
      <h1>Welcome to TimeWise!</h1>

      <p>
        Add your todos here and track their
        progress.
      </p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          add();
        }}
      >
        <input
          id="greet-input"
          value={name}
          onChange={(e) =>
            setName(e.currentTarget.value)
          }
          placeholder="Enter a name..."
        />
        <select
          name="pets"
          id="pet-select"
          value={state}
          onChange={(e) => {
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

      {items.length > 0 && (
        <>
          <h2>Items</h2>
          <ul>
            {items.map((item, index) => (
              <li key={index}>
                {item.title} -{" "}
                {StateLabel[item.state]}
              </li>
            ))}
          </ul>
        </>
      )}

      <button onClick={clear}>Clear</button>
      <button onClick={remove}>
        Remove Database
      </button>
    </div>
  );
}

export default App;

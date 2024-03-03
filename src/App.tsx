import { useEffect, useState } from "react";
import {
  InvokeArgs,
  invoke,
} from "@tauri-apps/api/tauri";
import { useNotification } from "./context/notifications";

// FLOW:
// 1. Fetch not-done items from the database
// 2. Show the list of items and allow to add new
// 3. Switch the screen to new step
// 4. Show the timeline
// 5. Pomodoro timing
// 6. Add items to timeslots
// 7. Start the day
// 8. On the main screen,
//    show the progress and the list fo the items (with actions - done, partially done, not done),
//    show timer and indicate pauses

async function fetch(
  operation: string,
  args?: InvokeArgs | undefined
): Promise<Item[]> {
  const response = await invoke<string>(
    operation,
    args
  );
  return JSON.parse(response);
}

export function Home() {
  const [toDoItems, setToDOItems] = useState<
    Item[]
  >([]);
  const [inProgressItems, setInProgressItems] =
    useState<Item[]>([]);

  useEffect(() => {
    updateData();
  }, []);

  function updateData() {
    fetch("fetch_by_state", {
      state: State.TODO,
    }).then((data) => setToDOItems(data));

    fetch("fetch_by_state", {
      state: State.IN_PROGRESS,
    }).then((data) => setInProgressItems(data));
  }

  function removeItem(title: string) {
    invoke("remove_item", { title }).then(() =>
      updateData()
    );
  }

  return (
    <div className="container">
      <h1>Welcome to TimeWise!</h1>
      <p>
        Add your todos here and track their
        progress.
      </p>
      <InProgressList
        items={inProgressItems}
        remove={removeItem}
      />
      <NewList
        items={toDoItems}
        onAction={updateData}
        remove={removeItem}
      />
    </div>
  );
}

export function ItemCard(props: {
  item: Item;
  remove: (title: string) => void;
}) {
  const { item, remove } = props;

  return (
    <div className="card">
      <div className="card-head">
        <span className="card-title">
          {item.title}
        </span>
        <span>{StateLabel[item.state]}</span>
      </div>
      <button
        onClick={() => remove(item.title)}
        className="remove-button"
      >
        Remove
      </button>
    </div>
  );
}

export function InProgressList(props: {
  items: Item[];
  remove: (title: string) => void;
}) {
  const { items, remove } = props;

  if (items.length === 0) {
    return null;
  }

  return (
    <div className="container">
      <h1>In Progress</h1>
      <ul>
        {items.map((item, index) => (
          <ItemCard
            key={index}
            item={item}
            remove={remove}
          />
        ))}
      </ul>
    </div>
  );
}

export function NewList(props: {
  items: Item[];
  remove: (title: string) => void;
  onAction: () => void;
}) {
  const [name, setName] = useState("");
  const [state, setState] = useState<State>(
    State.TODO
  );
  const { addNotification } = useNotification();
  const { items, onAction, remove } = props;

  async function add() {
    await invoke("add", { name, state })
      .then(() => {
        setName("");
        setState(State.TODO);
        addNotification("Item added!", 3000);
      })
      .finally(() => onAction());
  }

  return (
    <div className="container">
      <h1>New</h1>
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
        <button
          type="submit"
          className="add-button"
        >
          Add
        </button>
      </form>

      {items.length > 0 && (
        <>
          <ul>
            {items.map((item, index) => (
              <ItemCard
                key={index}
                item={item}
                remove={remove}
              />
            ))}
          </ul>
        </>
      )}
    </div>
  );
}

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

export default Home;

import { createContext, useContext } from "react";

export const NotificationContext = createContext({
  addNotification: (
    _message: string,
    _duration: number
  ) => {},
  removeNotification: (_id: number) => {},
});

export const useNotification = () =>
  useContext(NotificationContext);

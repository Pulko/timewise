import React, { useState } from "react";
import { UserNotification } from "../../types/notification";
import { NotificationContext } from "../../context/notifications";

import "./index.css";

function LayoutNotifications(
  props: React.PropsWithChildren
) {
  const [notifications, setNotifications] =
    useState<Array<UserNotification>>([]);

  const addNotification = (
    message: string,
    duration = 3000
  ) => {
    const id =
      notifications[notifications.length - 1]
        ?.id || 0 + 1;
    const notification = {
      id,
      message,
      duration,
    };

    setNotifications([
      ...notifications,
      notification,
    ]);

    setTimeout(() => {
      removeNotification(id);
    }, duration);
  };

  const removeNotification = (
    idToRemove: UserNotification["id"]
  ) => {
    setNotifications(
      notifications.filter(
        (notification) =>
          notification.id !== idToRemove
      )
    );
  };

  const contextValue = {
    addNotification,
    removeNotification,
  };

  return (
    <NotificationContext.Provider
      value={contextValue}
    >
      {props.children}
      <div className="notification-container">
        {notifications.map((notification) => (
          <div
            key={notification.id}
            className="notification"
          >
            {notification.message}
          </div>
        ))}
      </div>
    </NotificationContext.Provider>
  );
}

export default LayoutNotifications;

import { useState } from "react";
import { getToken, getUser, clearAuth } from "./api";
import Auth from "./Auth.jsx";
import ChatApp from "./ChatApp.jsx";

export default function App() {
  const [token, setToken] = useState(getToken());
  const [user, setUser] = useState(getUser());

  if (!token || !user) {
    return (
      <Auth
        onAuth={(t, u) => {
          setToken(t);
          setUser(u);
        }}
      />
    );
  }

  return (
    <ChatApp
      user={user}
      onLogout={() => {
        clearAuth();
        setToken(null);
        setUser(null);
      }}
    />
  );
}

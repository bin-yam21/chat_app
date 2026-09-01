const BASE = "http://localhost:3000/api/v1";
export const WS_BASE = "ws://localhost:3000/api/v1";

export const getToken = () => localStorage.getItem("rc_token");
export const getUser = () => {
  const u = localStorage.getItem("rc_user");
  return u ? JSON.parse(u) : null;
};
export const saveAuth = (token, user) => {
  localStorage.setItem("rc_token", token);
  localStorage.setItem("rc_user", JSON.stringify(user));
};
export const clearAuth = () => {
  localStorage.removeItem("rc_token");
  localStorage.removeItem("rc_user");
};

async function req(path, opts = {}) {
  const token = getToken();
  const res = await fetch(BASE + path, {
    ...opts,
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(opts.headers || {}),
    },
  });
  if (!res.ok) throw new Error((await res.text()) || res.statusText);
  if (res.status === 204) return null;
  const ct = res.headers.get("content-type") || "";
  return ct.includes("application/json") ? res.json() : res.text();
}

export const api = {
  register: (body) => req("/register", { method: "POST", body: JSON.stringify(body) }),
  login: (body) => req("/login", { method: "POST", body: JSON.stringify(body) }),
  rooms: () => req("/rooms"),
  createRoom: (name, created_by) =>
    req("/rooms", { method: "POST", body: JSON.stringify({ name, created_by }) }),
  messages: (roomId) => req(`/rooms/${roomId}/messages`),
  users: () => req("/users"),
};

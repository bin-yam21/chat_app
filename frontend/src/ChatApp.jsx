import { useEffect, useRef, useState } from "react";
import { api, getToken, WS_BASE } from "./api";

const AVATAR_COLORS = [
  "bg-rust", "bg-emerald-500", "bg-sky-500", "bg-violet-500",
  "bg-amber-500", "bg-pink-500", "bg-teal-500", "bg-indigo-500",
];
const colorFor = (id = "") => {
  let h = 0;
  for (let i = 0; i < id.length; i++) h = (h * 31 + id.charCodeAt(i)) % AVATAR_COLORS.length;
  return AVATAR_COLORS[h];
};
const initials = (name = "?") => name.slice(0, 2).toUpperCase();
const timeOf = (iso) => {
  try {
    return new Date(iso).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
  } catch {
    return "";
  }
};

export default function ChatApp({ user, onLogout }) {
  const [rooms, setRooms] = useState([]);
  const [activeRoom, setActiveRoom] = useState(null);
  const [messages, setMessages] = useState([]);
  const [text, setText] = useState("");
  const [members, setMembers] = useState({}); // id -> username
  const [connected, setConnected] = useState(false);
  const [newRoom, setNewRoom] = useState("");
  const wsRef = useRef(null);
  const scrollRef = useRef(null);

  // Load rooms + members
  useEffect(() => {
    api.rooms().then(setRooms).catch(() => setRooms([]));
    api
      .users()
      .then((list) => {
        const map = {};
        (list || []).forEach((u) => (map[u.id] = u.username));
        setMembers(map);
      })
      .catch(() => {}); // admin-only; ignore if forbidden
  }, []);

  // When a room is selected: load history + open WebSocket
  useEffect(() => {
    if (!activeRoom) return;
    setMessages([]);
    api.messages(activeRoom.id).then(setMessages).catch(() => setMessages([]));

    const ws = new WebSocket(`${WS_BASE}/ws/${activeRoom.id}?token=${getToken()}`);
    ws.onopen = () => setConnected(true);
    ws.onclose = () => setConnected(false);
    ws.onmessage = (e) => {
      try {
        const msg = JSON.parse(e.data);
        setMessages((prev) => (prev.some((m) => m.id === msg.id) ? prev : [...prev, msg]));
      } catch {}
    };
    wsRef.current = ws;
    return () => ws.close();
  }, [activeRoom]);

  useEffect(() => {
    scrollRef.current?.scrollTo({ top: scrollRef.current.scrollHeight, behavior: "smooth" });
  }, [messages]);

  const send = () => {
    const body = text.trim();
    if (!body || wsRef.current?.readyState !== WebSocket.OPEN) return;
    wsRef.current.send(JSON.stringify({ user_id: user.id, content: body }));
    setText("");
  };

  const createRoom = async () => {
    const name = newRoom.trim();
    if (!name) return;
    try {
      const room = await api.createRoom(name, user.id);
      setRooms((r) => [...r, room]);
      setNewRoom("");
      setActiveRoom(room);
    } catch {}
  };

  const nameFor = (id) => (id === user.id ? user.username : members[id] || "Member");

  return (
    <div className="flex h-screen bg-ink text-white">
      {/* Sidebar */}
      <aside className="flex w-72 flex-col border-r border-line bg-panel">
        <div className="flex items-center gap-3 border-b border-line px-5 py-4">
          <div className="grid size-9 place-items-center rounded-lg bg-rust text-lg">🦀</div>
          <span className="font-display text-lg font-bold">RustChat</span>
        </div>

        <div className="px-4 py-4">
          <div className="flex gap-2">
            <input
              value={newRoom}
              onChange={(e) => setNewRoom(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && createRoom()}
              placeholder="New room…"
              className="min-w-0 flex-1 rounded-lg border border-line bg-panel2 px-3 py-2 text-sm placeholder:text-white/30 focus:border-rust focus:outline-none"
            />
            <button
              onClick={createRoom}
              className="grid size-9 shrink-0 place-items-center rounded-lg bg-rust text-lg font-bold hover:bg-rust-dark"
            >
              +
            </button>
          </div>
        </div>

        <p className="px-5 pb-2 text-xs font-semibold uppercase tracking-wider text-white/40">
          Rooms
        </p>
        <nav className="no-scrollbar flex-1 space-y-1 overflow-y-auto px-3">
          {rooms.map((room) => (
            <button
              key={room.id}
              onClick={() => setActiveRoom(room)}
              className={`flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-left transition ${
                activeRoom?.id === room.id
                  ? "bg-rust/15 text-white"
                  : "text-white/60 hover:bg-panel2 hover:text-white"
              }`}
            >
              <span className="text-white/40">#</span>
              <span className="truncate font-medium">{room.name}</span>
            </button>
          ))}
          {rooms.length === 0 && (
            <p className="px-3 py-6 text-center text-sm text-white/30">
              No rooms yet. Create one above.
            </p>
          )}
        </nav>

        <div className="flex items-center gap-3 border-t border-line px-4 py-3">
          <div className={`grid size-9 place-items-center rounded-full text-sm font-bold ${colorFor(user.id)}`}>
            {initials(user.username)}
          </div>
          <div className="min-w-0 flex-1">
            <p className="truncate text-sm font-semibold">{user.username}</p>
            <p className="text-xs text-white/40">{user.role}</p>
          </div>
          <button onClick={onLogout} className="text-xs text-white/40 hover:text-rust">
            Log out
          </button>
        </div>
      </aside>

      {/* Main */}
      <main className="flex flex-1 flex-col">
        {activeRoom ? (
          <>
            <header className="flex items-center justify-between border-b border-line px-6 py-4">
              <div className="flex items-center gap-2">
                <span className="text-xl text-white/40">#</span>
                <h2 className="font-display text-lg font-semibold">{activeRoom.name}</h2>
              </div>
              <span className="flex items-center gap-2 text-xs text-white/50">
                <span className={`size-2 rounded-full ${connected ? "bg-emerald-400" : "bg-white/30"}`} />
                {connected ? "Live" : "Connecting…"}
              </span>
            </header>

            <div ref={scrollRef} className="no-scrollbar flex-1 space-y-4 overflow-y-auto px-6 py-6">
              {messages.map((m) => {
                const mine = m.user_id === user.id;
                const name = nameFor(m.user_id);
                return (
                  <div key={m.id} className={`flex gap-3 ${mine ? "flex-row-reverse" : ""}`}>
                    <div className={`grid size-9 shrink-0 place-items-center rounded-full text-xs font-bold ${colorFor(m.user_id)}`}>
                      {initials(name)}
                    </div>
                    <div className={`max-w-[70%] ${mine ? "items-end text-right" : ""}`}>
                      <div className={`mb-1 flex items-center gap-2 text-xs text-white/40 ${mine ? "justify-end" : ""}`}>
                        <span className="font-semibold text-white/70">{mine ? "You" : name}</span>
                        <span>{timeOf(m.created_at)}</span>
                      </div>
                      <div
                        className={`inline-block rounded-2xl px-4 py-2.5 text-[0.95rem] leading-relaxed ${
                          mine
                            ? "rounded-tr-sm bg-rust text-white"
                            : "rounded-tl-sm bg-panel2 text-white/90"
                        }`}
                      >
                        {m.content}
                      </div>
                    </div>
                  </div>
                );
              })}
              {messages.length === 0 && (
                <div className="flex h-full items-center justify-center text-white/30">
                  No messages yet — say hello 👋
                </div>
              )}
            </div>

            <div className="border-t border-line px-6 py-4">
              <div className="flex items-center gap-3 rounded-xl border border-line bg-panel px-4 py-2">
                <input
                  value={text}
                  onChange={(e) => setText(e.target.value)}
                  onKeyDown={(e) => e.key === "Enter" && send()}
                  placeholder={`Message #${activeRoom.name}`}
                  className="flex-1 bg-transparent py-2 text-white placeholder:text-white/30 focus:outline-none"
                />
                <button
                  onClick={send}
                  className="rounded-lg bg-rust px-5 py-2 text-sm font-semibold hover:bg-rust-dark"
                >
                  Send
                </button>
              </div>
            </div>
          </>
        ) : (
          <div className="flex flex-1 flex-col items-center justify-center text-center text-white/40">
            <div className="mb-4 grid size-16 place-items-center rounded-2xl bg-panel2 text-3xl">🦀</div>
            <h2 className="font-display text-2xl font-semibold text-white/70">Welcome to RustChat</h2>
            <p className="mt-2 max-w-sm">Pick a room on the left, or create a new one to start chatting in real time.</p>
          </div>
        )}
      </main>
    </div>
  );
}

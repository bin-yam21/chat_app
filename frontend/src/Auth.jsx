import { useState } from "react";
import { api, saveAuth } from "./api";

export default function Auth({ onAuth }) {
  const [mode, setMode] = useState("login");
  const [form, setForm] = useState({ username: "", email: "", password: "" });
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const submit = async (e) => {
    e.preventDefault();
    setError("");
    setLoading(true);
    try {
      const res =
        mode === "login"
          ? await api.login({ username: form.username, password: form.password })
          : await api.register({
              username: form.username,
              email: form.email || null,
              password: form.password,
            });
      saveAuth(res.token, res.user);
      onAuth(res.token, res.user);
    } catch {
      setError(
        mode === "login"
          ? "Invalid username or password."
          : "Could not register — try a different username."
      );
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex min-h-screen bg-ink text-white">
      {/* Brand panel */}
      <div className="relative hidden w-1/2 overflow-hidden lg:block">
        <div className="absolute inset-0 bg-gradient-to-br from-rust/30 via-panel to-ink" />
        <div className="absolute -left-20 top-1/3 size-96 rounded-full bg-rust/20 blur-3xl" />
        <div className="relative flex h-full flex-col justify-between p-14">
          <div className="flex items-center gap-3">
            <div className="grid size-11 place-items-center rounded-xl bg-rust text-2xl">🦀</div>
            <span className="font-display text-2xl font-bold tracking-tight">RustChat</span>
          </div>
          <div>
            <h1 className="font-display text-5xl font-bold leading-tight">
              Real-time chat,
              <br />
              <span className="text-rust">powered by Rust.</span>
            </h1>
            <p className="mt-6 max-w-md text-lg text-white/60">
              A blazing-fast messaging backend built with Axum, SQLx and
              WebSockets — with JWT auth and live broadcast to every room.
            </p>
            <div className="mt-8 flex gap-3 text-sm text-white/50">
              {["Axum", "SQLx", "PostgreSQL", "WebSockets", "JWT"].map((t) => (
                <span key={t} className="rounded-full border border-line px-3 py-1">
                  {t}
                </span>
              ))}
            </div>
          </div>
          <p className="text-sm text-white/40">Documented with OpenAPI · /docs</p>
        </div>
      </div>

      {/* Form panel */}
      <div className="flex w-full items-center justify-center p-6 lg:w-1/2">
        <div className="w-full max-w-sm">
          <div className="mb-8 flex items-center gap-3 lg:hidden">
            <div className="grid size-10 place-items-center rounded-xl bg-rust text-xl">🦀</div>
            <span className="font-display text-xl font-bold">RustChat</span>
          </div>

          <h2 className="font-display text-3xl font-bold">
            {mode === "login" ? "Welcome back" : "Create account"}
          </h2>
          <p className="mt-2 text-white/50">
            {mode === "login"
              ? "Sign in to jump back into the conversation."
              : "Join and start chatting in real time."}
          </p>

          <form onSubmit={submit} className="mt-8 space-y-4">
            <Field
              label="Username"
              value={form.username}
              onChange={(v) => setForm({ ...form, username: v })}
              placeholder="abebe"
            />
            {mode === "register" && (
              <Field
                label="Email (optional)"
                value={form.email}
                onChange={(v) => setForm({ ...form, email: v })}
                placeholder="abebe@example.com"
              />
            )}
            <Field
              label="Password"
              type="password"
              value={form.password}
              onChange={(v) => setForm({ ...form, password: v })}
              placeholder="••••••••"
            />

            {error && <p className="text-sm text-rust">{error}</p>}

            <button
              type="submit"
              disabled={loading}
              className="w-full rounded-xl bg-rust py-3 font-semibold text-white transition hover:bg-rust-dark disabled:opacity-60"
            >
              {loading ? "Please wait…" : mode === "login" ? "Sign in" : "Create account"}
            </button>
          </form>

          <p className="mt-6 text-center text-sm text-white/50">
            {mode === "login" ? "New here?" : "Already have an account?"}{" "}
            <button
              onClick={() => {
                setMode(mode === "login" ? "register" : "login");
                setError("");
              }}
              className="font-semibold text-rust hover:underline"
            >
              {mode === "login" ? "Create an account" : "Sign in"}
            </button>
          </p>
        </div>
      </div>
    </div>
  );
}

function Field({ label, value, onChange, type = "text", placeholder }) {
  return (
    <label className="block">
      <span className="mb-1.5 block text-sm font-medium text-white/70">{label}</span>
      <input
        type={type}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        className="w-full rounded-xl border border-line bg-panel px-4 py-3 text-white placeholder:text-white/30 focus:border-rust focus:outline-none focus:ring-2 focus:ring-rust/30"
      />
    </label>
  );
}

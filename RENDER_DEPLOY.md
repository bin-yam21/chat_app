# Deploying Rust Chat — Backend on Render, Frontend on Vercel

This guide takes the app from local-only to a public live demo:

- **Backend** (Axum + Postgres) → **Render** (Docker web service + managed Postgres)
- **Frontend** (Vite/React) → **Vercel** (static site pointed at the Render API)

The code is already deploy-ready: the server binds to `$PORT`, runs its own
migrations on startup, and reads all config from environment variables. Nothing
below needs code changes — just clicking through the two dashboards.

---

## 1. Push the repo

Make sure this branch is on GitHub (`github.com/bin-yam21/chat_app`):

```bash
git push origin rust-chat-frontend-and-docs
```

> You can deploy from this branch, or merge it into `main` first and deploy
> from `main`. Render lets you pick the branch either way.

---

## 2. Backend + database on Render (Blueprint)

The repo ships a `render.yaml` that provisions **both** the Postgres database
and the web service in one step.

1. Go to <https://dashboard.render.com> → **New +** → **Blueprint**.
2. Connect your GitHub and select the **`chat_app`** repo.
3. Pick the branch (`rust-chat-frontend-and-docs` or `main`).
4. Render reads `render.yaml` and shows two resources to create:
   - `chat-app-db` — free Postgres
   - `chat-app-api` — Docker web service
   `DATABASE_URL` is wired from the DB automatically and a `JWT_SECRET` is
   generated for you. Click **Apply**.
5. First build takes a while (Rust compiles from scratch, ~5–10 min). Watch the
   logs; you want to see `✅ Database migrations are up to date` then
   `Server running on 0.0.0.0:...`.
6. When it's live, your API base URL is something like:
   **`https://chat-app-api.onrender.com`**

Quick check — open these in a browser:
- `https://chat-app-api.onrender.com/openapi.json` → returns JSON
- `https://chat-app-api.onrender.com/docs` → Swagger UI

> **Free-tier note:** free web services sleep after ~15 min idle, so the first
> request after a pause takes ~30–60s to wake. Fine for a portfolio demo; on the
> portfolio you can mention "first load may take a moment (free tier)".

### Manual alternative (no Blueprint)
If you'd rather not use the Blueprint: create a **PostgreSQL** instance first,
then a **Web Service** → **Docker** from the repo, and add two env vars on the
service: `DATABASE_URL` (the DB's *Internal* connection string) and
`JWT_SECRET` (any long random string). Render sets `PORT` itself.

---

## 3. Frontend on Vercel

The frontend now reads the backend origin from `VITE_API_URL` (see
`frontend/.env.example`) and derives the WebSocket URL from it (`https`→`wss`).

1. Go to <https://vercel.com> → **Add New** → **Project** → import the same repo.
2. Set **Root Directory** to `frontend`.
3. Framework preset: **Vite**. Build command `npm run build`, output `dist`
   (Vercel detects these automatically).
4. Add an **Environment Variable**:
   - Name: `VITE_API_URL`
   - Value: `https://chat-app-api.onrender.com` *(your Render URL, no trailing slash)*
5. **Deploy.** You'll get a URL like `https://rust-chat.vercel.app`.

Because the backend already sends permissive CORS (`Access-Control-Allow-Origin: *`),
no extra backend config is needed for the browser to call it.

---

## 4. Smoke-test the live demo

On the Vercel URL:
1. Register a user, then register a second user in an incognito window.
2. Create a room, send messages — they should appear in real time in both
   windows (that's the WebSocket over `wss://` working).

---

## 5. Add it to the portfolio

Once the demo is live, wire the URL into the portfolio so the live-demo section
lights up. In `personal portfolio/my-portfolio/_data/data.ts`, on the
`rust-chat-service` entry, uncomment/set:

```ts
link: "https://rust-chat.vercel.app",
```

That turns on the green "Live Demo" badge on the card and the "Launch Live
Interactive Demo" button in the project's demo section.

---

## Troubleshooting

- **Build fails on `COPY .env`** — you're on an old Dockerfile; pull latest.
  The current Dockerfile no longer copies `.env`.
- **App crashes: `DATABASE_URL must be set`** — the env var isn't attached to the
  web service. On Blueprint it's automatic; on manual setup add it yourself.
- **DB connection/TLS errors** — use the database's **Internal** connection
  string (same-region private network). If you must use the external one, append
  `?sslmode=require` to `DATABASE_URL`.
- **WebSocket won't connect from the browser** — confirm `VITE_API_URL` starts
  with `https://` (so the socket becomes `wss://`); browsers block `ws://` from
  an `https://` page.

# 🦀 Git-Backed Markdown Blog API & Client

A modern, high-performance blog platform powered by a **Rust + Axum** backend and a minimal **React + TypeScript + Vite** frontend. The backend reads and renders Markdown articles directly from a local Git-backed directory on demand.

Designed with clear architectural mappings for experienced **Node.js, Express, and NestJS** developers transitioning to Rust.

---

## 📑 Table of Contents
- [Architecture & Mental Model for Node.js Developers](#-architecture--mental-model-for-nodejs-developers)
- [Project Structure](#-project-structure)
- [Quick Start](#-quick-start)
  - [Option 1: One-Go Local Script (Recommended)](#option-1-one-go-local-script-recommended)
  - [Option 2: Docker & Docker Compose](#option-2-docker--docker-compose)
  - [Option 3: Manual Execution](#option-3-manual-execution)
- [API Specification](#-api-specification)
- [How the Git-Backed Content Engine Works](#-how-the-git-backed-content-engine-works)
- [Security](#-security)

---

## 🧠 Architecture & Mental Model for Node.js Developers

If you are coming from NestJS or Express, here is how Axum implements familiar web concepts with compile-time safety:

```
┌─────────────────────────────────────────────────────────────┐
│                       HTTP Request                          │
└──────────────────────────────┬──────────────────────────────┘
                               │
                ┌──────────────▼──────────────┐
                │   Tower Middleware (CORS)   │
                └──────────────┬──────────────┘
                               │
            ┌──────────────────▼──────────────────┐
            │   Axum Extractors                   │
            │   - Path(slug)       [@Param]       │
            │   - State(app_state) [@Inject]      │
            └──────────────────┬──────────────────┘
                               │
            ┌──────────────────▼──────────────────┐
            │   Tokio Non-Blocking I/O            │
            │   (tokio::fs::read_to_string)       │
            └──────────────────┬──────────────────┘
                               │
            ┌──────────────────▼──────────────────┐
            │   Result<Json<PostDetail>, AppError>│
            │   - Ok(Json)    -> 200 OK           │
            │   - Err(AppErr) -> IntoResponse     │
            └─────────────────────────────────────┘
```

| Concept | Node.js (Express / NestJS) | Rust (Axum) |
| :--- | :--- | :--- |
| **Dependency Injection** | NestJS `@Injectable()` / `private readonly service: PostService` | **`axum::extract::State<AppState>`** wrapped in an `Arc<T>` (Atomic Reference Counter) for safe multi-threaded sharing |
| **Route Parameters** | `req.params.slug` / NestJS `@Param('slug')` | **`Path(slug): Path<String>`** (Type-safe extractor) |
| **Request Payloads** | `req.body` / NestJS `@Body() dto: PostDto` | **`Json(payload): Json<PostDto>`** (Parsed via Serde) |
| **Asynchronous I/O** | `fs.promises.readFile()` (Node libuv threadpool) | **`tokio::fs::read_to_string().await`** (Tokio worker threadpool) |
| **Centralized Errors** | Express `(err, req, res, next)` / NestJS `ExceptionFilter` | **`AppError` enum implementing `IntoResponse`** |
| **DTOs & Validation** | TypeScript `interface` + `zod` / `class-validator` | **Rust `struct` + `#[derive(Serialize, Deserialize)]`** |

---

## 📂 Project Structure

```text
.
├── backend/
│   ├── Cargo.toml               # Cargo dependencies (Axum 0.7, Tokio, Serde, Pulldown-Cmark)
│   ├── Dockerfile               # Multi-stage release container build
│   ├── content/posts/           # Git-backed Markdown storage
│   │   ├── hello-world.md
│   │   └── rust-for-ts-devs.md
│   └── src/
│       ├── main.rs              # Tokio runtime bootstrap, port binding, and env fallbacks
│       ├── lib.rs               # Router assembly, AppState (DI), and CORS configuration
│       ├── error.rs             # Centralized AppError enum with IntoResponse
│       ├── handlers/
│       │   ├── mod.rs
│       │   └── post.rs          # API controllers (list_posts, get_post_by_slug)
│       └── models/
│           ├── mod.rs
│           └── post.rs          # DTOs: PostSummary & PostDetail
├── frontend/
│   ├── package.json             # React 19, TypeScript, Vite, react-markdown
│   ├── Dockerfile               # Multi-stage container build (Node -> Nginx)
│   ├── nginx.conf               # Production Nginx reverse-proxy & SPA routing
│   └── src/
│       ├── App.tsx              # Master-detail blog viewer
│       └── main.tsx
├── docker-compose.yml           # Multi-container orchestration with live content mount
├── run-dev.ps1                  # One-click local startup (PowerShell)
├── run-dev.sh                   # One-click local startup (Bash)
├── docker-run.ps1               # One-click Docker launch (PowerShell)
└── docker-run.sh                # One-click Docker launch (Bash)
```

---

## 🚀 Quick Start

### Prerequisites
- **Rust & Cargo** (1.75+): [https://rustup.rs](https://rustup.rs)
- **Node.js** (v18+) & **pnpm**: `npm install -g pnpm`
- *(Optional for Docker)*: **Docker Desktop**

---

### Option 1: One-Go Local Script (Recommended)

Starts both the Axum backend (`http://127.0.0.1:3000`) and Vite frontend (`http://localhost:5173`) concurrently. Pressing `Ctrl+C` cleanly terminates both processes.

**Windows (PowerShell):**
```powershell
.\run-dev.ps1
```

**macOS / Linux (Bash):**
```bash
chmod +x run-dev.sh
./run-dev.sh
```

---

### Option 2: Docker & Docker Compose

Runs both services in isolated containers:
- **Frontend + Nginx Reverse Proxy**: [http://localhost:8080](http://localhost:8080)
- **Backend API**: [http://localhost:3000/api/posts](http://localhost:3000/api/posts)
- The local `./backend/content/posts` folder is **volume-mounted**, so saving markdown edits locally updates the blog in Docker in real-time.

**Windows (PowerShell):**
```powershell
.\docker-run.ps1
```

**macOS / Linux (Bash):**
```bash
chmod +x docker-run.sh
./docker-run.sh
```

*(Or invoke Docker directly)*:
```bash
docker compose up --build
```

---

### Option 3: Manual Execution

#### 1. Backend
```bash
cd backend
cargo run
```
*Listens on `http://127.0.0.1:3000` (or the address configured in `BIND_ADDR`).*

#### 2. Frontend
```bash
cd frontend
pnpm install
pnpm dev
```
*Listens on `http://localhost:5173`.*

---

## 📡 API Specification

### 1. List Posts
```http
GET /api/posts
```
**Response (`200 OK`):**
```json
[
  {
    "slug": "hello-world",
    "title": "hello world"
  },
  {
    "slug": "rust-for-ts-devs",
    "title": "rust for ts devs"
  }
]
```

### 2. Get Post by Slug
```http
GET /api/posts/:slug
```
**Response (`200 OK`):**
```json
{
  "slug": "rust-for-ts-devs",
  "title": "rust for ts devs",
  "markdown": "# Rust for TypeScript Developers\n...",
  "html": "<h1>Rust for TypeScript Developers</h1>\n..."
}
```

### 3. Error Responses
Errors are standardized through `AppError` and `IntoResponse`:

- **Not Found (`404`):**
  ```json
  {
    "error": "Post 'missing-article' was not found",
    "status": 404
  }
  ```
- **Invalid Path / Traversal Attempt (`400`):**
  ```json
  {
    "error": "Invalid slug characters",
    "status": 400
  }
  ```

---

## 📝 How the Git-Backed Content Engine Works

1. **Zero Database Dependency**: All posts are plain `.md` files residing in `backend/content/posts/`.
2. **On-Demand Asynchronous Reads**: When a request arrives, `handlers::post::get_post_by_slug` reads the file off disk asynchronously using Tokio's non-blocking filesystem API (`tokio::fs::read_to_string`).
3. **Instant Publish**: Because posts are read on-demand rather than pre-compiled into memory, simply pulling your Git repository (`git pull`) or adding a new `.md` file updates the API instantly with **zero downtime** and **no server restart required**.

---

## 🛡️ Security

- **Path Traversal Protection**: Slugs are strictly sanitized in `handlers/post.rs`. Any path containing `..`, `/`, or `\` is rejected immediately with HTTP 400 before touching the filesystem.
- **CORS**: Configured with `tower-http` to allow cross-origin requests from the Vite development server. In Docker, Nginx acts as a reverse proxy on the same origin (`/api/`), avoiding CORS altogether in production.
- **Strict Typing**: Struct serialization is verified at compile-time by `serde`, preventing prototype pollution and unhandled schema drift.


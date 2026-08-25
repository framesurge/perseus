# Perseus Form with SQLite Database

A complete example of handling form submissions with database persistence in Perseus.

## Features

- ✅ Create users with name and email
- ✅ List all registered users
- ✅ Delete users
- ✅ SQLite database backend
- ✅ REST API with Axum
- ✅ Form validation
- ✅ Real-time UI updates
- ✅ Error handling

## Project Structure

```
form_with_db/
├── src/
│   ├── main.rs              # Server setup with API routes & database
│   └── templates/
│       └── index.rs         # Frontend form and user list
├── Cargo.toml
└── README.md
```

## How It Works

### 1. Database (SQLite)

The database is initialized on server startup with a `users` table:

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
)
```

### 2. API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/users` | Get all users |
| POST | `/api/users` | Create new user |
| DELETE | `/api/users/:id` | Delete user by ID |

### 3. Frontend

The frontend uses:
- **Sycamore signals** for reactive state
- **gloo-net** for HTTP requests
- **Indexed list** for rendering users
- **Form validation** before submission

## Running the Example

```bash
cd examples/demos/form_with_db

# Run in development mode
perseus serve -w
```

Then open http://localhost:8080

## Database File

The SQLite database file `users.db` will be created in the project root when you first run the app.

## Key Code Sections

### Server-Side: API Handler

```rust
async fn create_user(
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse>, (StatusCode, String)> {
    // Insert user into database
    sqlx::query!(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        payload.name,
        payload.email
    )
    .execute(pool.as_ref())
    .await?;

    // Return success response
}
```

### Client-Side: Form Submission

```rust
let submit_form = move |_| {
    spawn_local(async move {
        let response = create_user(name_val, email_val).await;
        // Handle response and update UI
    });
};
```

### Client-Side: API Call

```rust
async fn create_user(name: String, email: String) -> Result<ApiResponse, String> {
    use gloo_net::http::Request;

    Request::post("/api/users")
        .json(&serde_json::json!({ "name": name, "email": email }))
        .send()
        .await?
        .json::<ApiResponse>()
        .await
}
```

## Testing

Try these scenarios:

1. **Add a user** - Fill in name and email, click "Add User"
2. **Duplicate email** - Try adding the same email twice (should fail)
3. **Empty fields** - Submit without filling fields (should fail)
4. **Delete user** - Click delete button on any user
5. **Refresh page** - Data persists because it's in SQLite

## Extending This Example

Ideas for enhancements:

- Add user update/edit functionality
- Add pagination for large user lists
- Add search/filter functionality
- Add more fields (phone, address, etc.)
- Add authentication
- Use PostgreSQL or MySQL instead of SQLite
- Add input validation on frontend and backend
- Add tests

## Dependencies

- **perseus** - Framework
- **axum** - Web server and routing
- **sqlx** - Database access
- **gloo-net** - HTTP requests (client-side)
- **serde/serde_json** - Serialization

## Notes

- The database is created automatically on first run
- Email uniqueness is enforced at database level
- All API calls include proper error handling
- The UI updates automatically after successful operations

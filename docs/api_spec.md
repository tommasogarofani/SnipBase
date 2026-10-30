# API Specification

Definition of the RESTful endpoints exposed by Snipbase's Rust backend. Base path: `/api`

## Endpoints

### Snippets

#### Get

- `GET /api/snippets`
  - Description: Returns a list of all snippets (with optional filters by tag or language).

- `GET /api/snippets/:id`
  - Description: Retrieves the details of a single snippet by ID.

#### Post

- `POST /api/snippets`
  - Description: Creates a new snippet and associates its tags (transactional many-to-many relationship management).
  - Request Body: `{ "title": "...", "description": "...", "content": "...", "language": "...", "tags": ["rust", "axum"] }`
  - Response: `id`

#### Put

- `PUT /api/snippets/:id`
  - Description: Updates an existing snippet and its tags.

#### Delete

- `DELETE /api/snippets/:id`
  - Description: Deletes a snippet.

### Tags

#### Get

- `GET /api/tags`
  - Description: Returns a list of all available tags.

#### Post

- `POST /api/tags`
  - Description: Creates a new tag.

#### Put

- `PUT /api/tags/:id`
  - Description: Updates an existing tag.

#### Delete

- `DELETE /api/tags/:id`
  - Description: Deletes a tag.

# API Specification

Definizione degli endpoint RESTful esposti dal backend Rust di Snipbase. Base path: `/api`

## Endpoints

### Snippets
- `GET /api/snippets`
  - Descrizione: Restituisce la lista di tutti gli snippet (con eventuali filtri per tag o linguaggio).
  - Risposta: `200 OK` (JSON array di snippet).

- `POST /api/snippets`
  - Descrizione: Crea un nuovo snippet e associa i relativi tag (gestione transazionale Many-to-Many).
  - Request Body: `{ "title": "...", "description": "...", "content": "...", "language": "...", "tags": ["rust", "axum"] }`
  - Risposta: `201 Created`

- `GET /api/snippets/:id`
  - Descrizione: Recupera il dettaglio di un singolo snippet tramite ID.

- `PUT /api/snippets/:id`
  - Descrizione: Aggiorna uno snippet esistente e i relativi tag.

- `DELETE /api/snippets/:id`
  - Descrizione: Elimina uno snippet.

### Tags
- `GET /api/tags`
  - Descrizione: Restituisce l'elenco di tutti i tag disponibili.
- `POST /api/tags`
  - Descrizione: Crea un nuovo tag.
# Database Architecture

This document outlines the database architecture for the SnipBase application.

## Tables

### Snippets managing tables

#### Snippets

- `id` (Primary Key, UUID): unique identifier for each snippet
- `title` (VARCHAR(255), NOT NULL): title of the snippet
- `description` (TEXT, optional): brief description of the snippet
- `content` (TEXT, NOT NULL): the actual code or text of the snippet
- `language` (VARCHAR(64), NOT NULL): programming language or format of the snippet (e.g., c, Rust, Markdown)
- `public` (BOOLEAN DEFAULT FALSE): boolean indicating whether the snippet is public or private (default false)
- `created_at` (TIMESTAMP WITH TIME ZONE): timestamp of when the snippet was created

#### tags

- `id` (Primary Key, UUID): unique identifier for each tag
- `name` (VARCHAR(255), NOT NULL): name of the tag (e.g., `axum`, `docker`, `async`)
- `color` (VARCHAR(7), optional): Hex color code for UI badges (e.g., `#FF5733`)

#### snippet_tags

- `snippet_id` (Foreign Key -> `snippets.id`, ON DELETE CASCADE): references the `id` in the `snippets` table
- `tag_id` (Foreign Key -> `tags.id`, ON DELETE CASCADE): references the `id` in the `tags` table
- `(snippet_id, tag_id)` (Primary Key): composite primary key to ensure uniqueness of snippet-tag pairs

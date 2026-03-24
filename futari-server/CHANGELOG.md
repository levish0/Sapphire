# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-24

### Added

- **Database Migrations**: Full SeaORM migration suite with UUIDv7 primary keys
  - `users` table with TOTP 2FA support, profile/banner images
  - `user_roles` table with `role` PostgreSQL enum (mod, admin) and optional expiration
  - `user_bans` table with optional expiration (NULL = permanent)
  - `user_preferences` table with JSONB value storage
  - `user_oauth_connections` table with `oauth_provider` enum
  - `posts` table — Twitter-style with repost/quote-repost support, CHECK constraints, denormalized counters (like, repost, quote, comment counts), and partial indexes
  - `comments` table — nested comments via self-referential `parent_comment_id`
  - `action_logs` table with `action_resource_type` PostgreSQL enum (post, comment)
  - `moderation_logs` table with `moderation_resource_type` PostgreSQL enum (user, post, comment, system)

- **Entity Layer** (`futari_entity`): SeaORM entity models for all tables with proper relations
  - `ActionResourceType` and `ModerationResourceType` active enums
  - `Role` enum with `display_priority()` method
  - `OAuthProvider` enum (Google, GitHub, Discord)

- **Constants** (`futari_constants`): Domain constants and enums
  - `ActionLogAction` enum for post/comment CRUD actions
  - `ModerationAction` enum for user/post/comment moderation
  - `UserPreferenceKey` enum
  - Cache key builders, storage key helpers, NATS subjects

- **Worker** (`futari_worker`): Background job processing via NATS JetStream
  - Email consumer (verification, password reset, email change)
  - User search index consumer (MeiliSearch)
  - Post search index consumer (MeiliSearch) with content search and `created_at` sorting
  - Reindex users consumer (batch cursor-based reindexing)
  - Cron scheduler with distributed Redis locks and heartbeat
    - Cleanup expired bans and roles (weekly)

- **Server** (`futari_server`): Axum-based API server skeleton
  - Repository layer for users, user roles, user bans, action logs, OAuth connections
  - Bridge to worker via NATS JetStream (email job publishing)
  - Event stream (SSE) publisher/subscriber infrastructure
  - Cursor-based pagination utilities

- **Configuration** (`futari_config`): Environment-based server configuration
- **Errors** (`futari_errors`): Typed error handling with HTTP status mapping
- **DTOs** (`futari_dto`): Request/response types for action logs and streaming

### Removed

- All V7 wiki-engine references and modules
  - Document/discussion/notification/points/IP ban/references systems
  - Document-specific search indexes, NATS streams, and cron jobs
  - Sitemap generation, orphaned blob/asset cleanup, backlink cache invalidation
  - R2 revision storage client (R2 assets client retained for user images)
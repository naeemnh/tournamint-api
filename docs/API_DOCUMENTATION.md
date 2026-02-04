# Tournamint API Documentation

## Overview
Complete tournament management system API with comprehensive features for matches, results, brackets, standings, user profiles, notifications, payments, and statistics.

## OpenAPI / Swagger

Interactive API documentation is available when the server is running:

- **Swagger UI**: `http://localhost:8080/swagger-ui/`
- **OpenAPI JSON**: `http://localhost:8080/api-docs/openapi.json`

Use **Authorize** in Swagger UI to add `Bearer <jwt_token>` for authenticated endpoints.

## Base URL
```
http://localhost:8080
```

## Authentication
All endpoints (except public profile viewing) require JWT authentication token in the Authorization header:
```
Authorization: Bearer <token>
```

---

## 1. Match Management APIs

### Create Match
- **POST** `/matches`
- **Body**: `CreateMatchRequest`
- **Response**: `Match`

### Get Match
- **GET** `/matches/{id}`
- **Response**: `Match`

### Update Match
- **PUT** `/matches/{id}`
- **Body**: `UpdateMatchRequest`
- **Response**: `Match`

### Delete Match
- **DELETE** `/matches/{id}`
- **Response**: Success message

### Get Matches by Tournament
- **GET** `/matches/tournament/{tournament_id}`
- **Response**: `Vec<Match>`

### Get Matches by Category
- **GET** `/matches/category/{category_id}`
- **Response**: `Vec<Match>`

### Update Match Status
- **PUT** `/matches/{id}/status`
- **Body**: `UpdateMatchStatusRequest`
- **Response**: `Match`

### Get Match Schedule
- **GET** `/matches/schedule`
- **Response**: `Vec<Match>`

### Validate Match Result Scores
- **GET** `/matches/{id}/results/validate`
- **Response**: Validation result for the match's result scores

---

## 2. Match Results APIs

### Submit Match Results
- **POST** `/match-results`
- **Body**: `CreateMatchResultRequest`
- **Response**: `MatchResult`

### Get Match Result
- **GET** `/match-results/{id}`
- **Response**: `MatchResult`

### Update Match Results
- **PUT** `/match-results/{id}`
- **Body**: `UpdateMatchResultRequest`
- **Response**: `MatchResult`

### Delete Match Results
- **DELETE** `/match-results/{id}`
- **Response**: Success message

### Get Results for Match
- **GET** `/match-results/match/{match_id}`
- **Response**: `Vec<MatchResult>`

### Get Match Score Summary
- **GET** `/match-results/match/{match_id}/summary`
- **Response**: `MatchScoreSummary`

### Bulk Create Results
- **POST** `/match-results/bulk`
- **Body**: `Vec<CreateMatchResultRequest>`
- **Response**: `Vec<MatchResult>`

---

## 3. Tournament Brackets APIs

### Get Tournament Bracket
- **GET** `/brackets/tournament/{tournament_id}`
- **Response**: `TournamentBracket`

### Get Category Bracket
- **GET** `/brackets/category/{category_id}`
- **Response**: `CategoryBracket`

### Generate Tournament Bracket
- **PUT** `/brackets/generate/{tournament_id}`
- **Body**: `GenerateBracketRequest`
- **Response**: `TournamentBracket`

---

## 4. Tournament Standings APIs

### Get Tournament Standings
- **GET** `/standings/tournament/{tournament_id}`
- **Response**: `Vec<TournamentStanding>`

### Get Category Standings
- **GET** `/standings/category/{category_id}`
- **Response**: `Vec<CategoryStanding>`

### Update Tournament Standings
- **PUT** `/standings/update/{tournament_id}`
- **Response**: `Vec<TournamentStanding>`

---

## 5. User Profile APIs

### Get Current User Profile
- **GET** `/profile`
- **Response**: `UserProfile`

### Update Current User Profile
- **PUT** `/profile`
- **Body**: `UpdateUserProfileRequest`
- **Response**: `UserProfile`

### Update User Preferences
- **POST** `/profile/preferences`
- **Body**: `UpdatePreferencesRequest`
- **Response**: `UserProfile`

### Update Notification Settings
- **POST** `/profile/notifications`
- **Body**: `UpdateNotificationPreferencesRequest`
- **Response**: `UserProfile`

### Update Privacy Settings
- **POST** `/profile/privacy`
- **Body**: `UpdatePrivacySettingsRequest`
- **Response**: `UserProfile`

### Upload/Update Avatar
- **POST** `/profile/avatar`
- **Body**: `UpdateAvatarRequest`
- **Response**: `UserProfile`

### Remove Avatar
- **DELETE** `/profile/avatar`
- **Response**: `UserProfile`

### Get Public Profile
- **GET** `/profile/{user_id}`
- **Response**: `PublicUserProfile`
- **Note**: No authentication required for public profiles

---

## 6. Notifications APIs

### Get User Notifications
- **GET** `/notifications`
- **Query Params**: 
  - `limit` (optional): Number of notifications
  - `offset` (optional): Pagination offset
- **Response**: `Vec<Notification>`

### Get Unread Notifications
- **GET** `/notifications/unread`
- **Response**: `Vec<Notification>`

### Mark Notification as Read
- **PUT** `/notifications/{id}/read`
- **Response**: `Notification`

### Mark All as Read
- **PUT** `/notifications/read-all`
- **Response**: Success message

### Delete Notification
- **DELETE** `/notifications/{id}`
- **Response**: Success message

### Send Notification (Admin)
- **POST** `/notifications/send`
- **Body**: `SendNotificationRequest`
- **Response**: `Notification`
- **Note**: Admin privileges required

### Get Unread Count
- **GET** `/notifications/count`
- **Response**: `{ "count": number }`

---

## 7. Payment APIs

### Process Payment
- **POST** `/payments/process`
- **Body**: `ProcessPaymentRequest`
- **Response**: `Payment`

### Get Payment Details
- **GET** `/payments/{id}`
- **Response**: `Payment`

### Get User Payments
- **GET** `/payments/user/{user_id}`
- **Response**: `Vec<Payment>`

### Get Tournament Payments
- **GET** `/payments/tournament/{tournament_id}`
- **Response**: `Vec<Payment>`

### Process Refund
- **PUT** `/payments/{id}/refund`
- **Body**: `RefundRequest`
- **Response**: `Payment`

### Get Payment Status
- **GET** `/payments/{id}/status`
- **Response**: `PaymentStatus`

### Get Payment Summary
- **GET** `/payments/summary`
- **Query Params**: 
  - `from_date` (optional)
  - `to_date` (optional)
- **Response**: `PaymentSummary`

---

## 8. Statistics APIs

### Get Player Statistics
- **GET** `/stats/player/{player_id}`
- **Query Params**: 
  - `tournament_id` (optional)
  - `from_date` (optional)
  - `to_date` (optional)
- **Response**: `PlayerStatistics`

### Get Team Statistics
- **GET** `/stats/team/{team_id}`
- **Query Params**: 
  - `tournament_id` (optional)
  - `from_date` (optional)
  - `to_date` (optional)
- **Response**: `TeamStatistics`

### Get Tournament Statistics
- **GET** `/stats/tournament/{tournament_id}`
- **Response**: `TournamentStatistics`

### Get Leaderboard
- **GET** `/stats/leaderboard`
- **Query Params**: 
  - `type`: player|team
  - `category` (optional)
  - `limit` (optional, default: 10)
- **Response**: `Vec<LeaderboardEntry>`

### Get Records
- **GET** `/stats/records`
- **Query Params**: 
  - `type`: highest_score|fastest_win|most_wins|longest_streak
  - `category` (optional)
  - `limit` (optional, default: 10)
- **Response**: `Vec<RecordEntry>`

---

## 9. Analytics APIs

### Get Analytics Dashboard
- **GET** `/analytics/dashboard`
- **Query Params**: 
  - `period`: daily|weekly|monthly|yearly
  - `from_date` (optional)
  - `to_date` (optional)
- **Response**: `AnalyticsDashboard`

---

## Data Models

### Match
```rust
{
  id: UUID,
  tournament_category_id: UUID,
  participant1_team_id?: UUID,
  participant1_player_id?: UUID,
  participant1_partner_id?: UUID,
  participant2_team_id?: UUID,
  participant2_player_id?: UUID,
  participant2_partner_id?: UUID,
  match_type: MatchType,
  match_status: MatchStatus,
  round_number?: i32,
  match_number?: i32,
  scheduled_date: DateTime,
  actual_start_date?: DateTime,
  actual_end_date?: DateTime,
  venue?: String,
  court_number?: String,
  winner_participant?: i32,
  is_draw: bool,
  referee_name?: String,
  umpire_name?: String,
  notes?: String,
  metadata?: JSON,
  created_at: DateTime,
  updated_at: DateTime
}
```

### MatchStatus
- `scheduled`
- `in_progress`
- `completed`
- `cancelled`
- `postponed`
- `forfeited`
- `bye`

### MatchType
- `group_stage`
- `round_of_128`
- `round_of_64`
- `round_of_32`
- `round_of_16`
- `quarter_final`
- `semi_final`
- `third_place`
- `final`
- `qualifying`
- `playoff`

### PaymentStatus
- `pending`
- `processing`
- `completed`
- `failed`
- `refunded`
- `partially_refunded`
- `cancelled`

### PaymentMethod
- `credit_card`
- `debit_card`
- `bank_transfer`
- `cash`
- `check`
- `online_payment`
- `other`

### NotificationType
- `tournament_update`
- `match_reminder`
- `result_posted`
- `registration_confirmed`
- `payment_received`
- `payment_failed`
- `team_invitation`
- `general`

---

## Error Responses

All error responses follow this format:
```json
{
  "status": "error",
  "message": "Error description",
  "data": null
}
```

### Common HTTP Status Codes
- `200` - Success
- `201` - Created
- `400` - Bad Request
- `401` - Unauthorized
- `403` - Forbidden
- `404` - Not Found
- `409` - Conflict
- `500` - Internal Server Error

---

## Testing

To test the APIs, ensure:
1. Database migrations are applied
2. JWT authentication is configured
3. Server is running on configured port

Example curl request:
```bash
curl -X GET http://localhost:8080/matches \
  -H "Authorization: Bearer <your-jwt-token>"
```

---

## Database Migrations

Run migrations before testing:
```bash
sqlx migrate run
```

---

## Notes
- All timestamps are in UTC
- UUIDs are used for all entity IDs
- JSONB fields allow flexible metadata storage
- Authentication required for all endpoints except public profiles
- Admin endpoints require additional privileges
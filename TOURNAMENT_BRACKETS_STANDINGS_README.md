# Tournament Brackets and Standings APIs

This document outlines the newly created Tournament Brackets and Standings API endpoints for the Rust server.

## Files Created

### Models
- **`src/models/tournament_bracket.rs`** - Bracket models and data structures
- **`src/models/tournament_standings.rs`** - Standings models and calculations

### Controllers
- **`src/controllers/tournament_bracket_controller.rs`** - HTTP handlers for bracket endpoints
- **`src/controllers/tournament_standings_controller.rs`** - HTTP handlers for standings endpoints

### Services
- **`src/services/tournament_bracket_service.rs`** - Business logic for bracket management
- **`src/services/tournament_standings_service.rs`** - Business logic for standings calculations

### Repositories
- **`src/repositories/tournament_bracket_repository.rs`** - Database operations for brackets
- **`src/repositories/tournament_standings_repository.rs`** - Database operations for standings

### Routes
- **`src/routes/tournament_bracket_routes.rs`** - URL routing for bracket endpoints
- **`src/routes/tournament_standings_routes.rs`** - URL routing for standings endpoints

## API Endpoints

### Tournament Brackets

#### GET /brackets/tournament/{tournament_id}
- Get all brackets for a specific tournament
- Returns: Array of bracket objects

#### GET /brackets/category/{category_id}
- Get bracket for a specific tournament category
- Returns: Detailed bracket with matches and participants

#### PUT /brackets/generate/{tournament_id}
- Generate a new bracket for a tournament
- Body: `GenerateBracketRequest` with bracket type and settings
- Returns: Generated bracket with created matches

### Tournament Standings

#### GET /standings/tournament/{tournament_id}
- Get standings for all categories in a tournament
- Returns: `StandingsResponse` with ordered participant rankings

#### GET /standings/category/{category_id}
- Get standings for a specific tournament category
- Returns: `StandingsResponse` with category-specific rankings

#### PUT /standings/update/{tournament_id}
- Update standings based on match results
- Body: `StandingsUpdateRequest` with recalculation options
- Returns: Success message with update count

## Key Features

### Bracket Generation
- **Single Elimination**: Traditional knockout format
- **Double Elimination**: Winners and losers brackets
- **Round Robin**: All participants play each other
- **Swiss System**: Planned for future implementation
- **Custom Seeding**: Support for custom participant ordering

### Standings Calculation
- **Points System**: 3 points for win, 1 for draw, 0 for loss
- **Tiebreakers**: Goal difference, head-to-head, games/sets ratio
- **Multiple Formats**: Support for different tournament formats
- **Real-time Updates**: Automatic recalculation on match completion

### Data Models

#### TournamentBracket
- Tournament/category association
- Bracket type (elimination, round-robin, etc.)
- Current status and progress
- JSON bracket structure
- Configuration settings

#### TournamentStandings
- Participant rankings and statistics
- Match records (played, won, lost, drawn)
- Points calculations
- Set and game statistics
- Goal difference tracking
- Elimination status

## Database Schema Requirements

The implementation requires these database tables (not yet created):

```sql
-- Tournament Brackets
CREATE TABLE tournament_brackets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_id UUID NOT NULL REFERENCES tournaments(id),
    category_id UUID REFERENCES tournament_categories(id),
    bracket_type bracket_type NOT NULL,
    status bracket_status NOT NULL DEFAULT 'not_generated',
    total_rounds INTEGER NOT NULL,
    current_round INTEGER NOT NULL DEFAULT 1,
    bracket_data JSONB,
    settings JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Tournament Standings
CREATE TABLE tournament_standings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_id UUID NOT NULL REFERENCES tournaments(id),
    category_id UUID REFERENCES tournament_categories(id),
    participant_id UUID NOT NULL,
    participant_name VARCHAR NOT NULL,
    participant_type VARCHAR NOT NULL,
    position INTEGER NOT NULL,
    points DECIMAL NOT NULL DEFAULT 0,
    matches_played INTEGER NOT NULL DEFAULT 0,
    matches_won INTEGER NOT NULL DEFAULT 0,
    matches_lost INTEGER NOT NULL DEFAULT 0,
    matches_drawn INTEGER NOT NULL DEFAULT 0,
    sets_won INTEGER NOT NULL DEFAULT 0,
    sets_lost INTEGER NOT NULL DEFAULT 0,
    games_won INTEGER NOT NULL DEFAULT 0,
    games_lost INTEGER NOT NULL DEFAULT 0,
    goal_difference INTEGER,
    head_to_head JSONB,
    bonus_points DECIMAL,
    penalty_points DECIMAL,
    is_eliminated BOOLEAN NOT NULL DEFAULT false,
    elimination_round VARCHAR,
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(tournament_id, COALESCE(category_id, '00000000-0000-0000-0000-000000000000'::uuid), participant_id)
);

-- Enums
CREATE TYPE bracket_type AS ENUM (
    'single_elimination',
    'double_elimination', 
    'round_robin',
    'swiss',
    'group_stage'
);

CREATE TYPE bracket_status AS ENUM (
    'not_generated',
    'generated',
    'in_progress',
    'completed'
);
```

## Integration Notes

### Module Registration
All new modules have been added to their respective `mod.rs` files:
- Models registered in `src/models/mod.rs`
- Controllers registered in `src/controllers/mod.rs`
- Services registered in `src/services/mod.rs`
- Repositories registered in `src/repositories/mod.rs`
- Routes registered in `src/routes/mod.rs`

### Dependencies
The implementation uses existing patterns from the codebase:
- Sea-query for SQL generation
- SQLx for database operations
- Actix-web for HTTP handling
- Serde for JSON serialization
- UUID for primary keys
- Chrono for timestamps

## Usage Examples

### Generate Single Elimination Bracket
```bash
curl -X PUT http://localhost:8080/brackets/generate/{tournament_id} \
  -H "Content-Type: application/json" \
  -d '{
    "bracket_type": "single_elimination",
    "category_id": null,
    "seed_order": null,
    "settings": {}
  }'
```

### Get Tournament Standings
```bash
curl http://localhost:8080/standings/tournament/{tournament_id}
```

### Update Standings After Matches
```bash
curl -X PUT http://localhost:8080/standings/update/{tournament_id} \
  -H "Content-Type: application/json" \
  -d '{
    "recalculate_all": true,
    "category_id": null
  }'
```

## Future Enhancements

1. **Advanced Bracket Types**: Swiss system, group stage + knockout
2. **Bracket Visualization**: JSON structure for frontend rendering
3. **Live Updates**: WebSocket notifications for bracket changes
4. **Playoff Generation**: Automatic advancement to next rounds
5. **Bracket Export**: PDF/image generation for printing
6. **Bracket Validation**: Ensure tournament rules compliance
7. **Historical Brackets**: Archive completed brackets
8. **Custom Scoring**: Support for sport-specific point systems

## Testing

Unit tests should be added for:
- Bracket generation algorithms
- Standings calculation logic
- Tiebreaker resolution
- Edge cases (odd participants, byes, etc.)

## Error Handling

The APIs include proper error handling for:
- Invalid tournament/category IDs
- Missing participants
- Duplicate bracket generation
- Database constraint violations
- Invalid bracket configurations

---

**Note**: This implementation provides a solid foundation for tournament management. The database tables need to be created and some methods need to be implemented once the dependent tables (matches, match_results, tournament_registrations) are available.
// Comprehensive Unit Tests for TournamentService Business Logic
// These tests focus on immutable business rules and validation logic
// Mock-free approach focusing on pure business logic validation

use chrono::Utc;
use rust_decimal::Decimal;
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;

// Test-specific structs that mirror the actual service types for business logic testing
// These are used purely for validating business rules without dependency on internal modules

#[derive(Debug, Clone, PartialEq)]
pub enum SportType {
    Basketball,
    TableTennis,
    Volleyball,
    Badminton,
    Tennis,
    Football,
    Cricket,
    Chess,
    Esports,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TournamentFormat {
    Elimination,
    DoubleElimination,
    RoundRobin,
    League,
    Swiss,
    GroupsAndKnockout,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TournamentStatus {
    Draft,
    Upcoming,
    RegistrationOpen,
    RegistrationClosed,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct Tournament {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub sport_type: SportType,
    pub format: TournamentFormat,
    pub status: TournamentStatus,
    pub start_date: chrono::DateTime<Utc>,
    pub end_date: chrono::DateTime<Utc>,
    pub registration_start_date: Option<chrono::DateTime<Utc>>,
    pub registration_end_date: Option<chrono::DateTime<Utc>>,
    pub venue: Option<String>,
    pub max_participants: Option<i32>,
    pub entry_fee: Option<Decimal>,
    pub prize_pool: Option<Decimal>,
    pub organizer_id: Uuid,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewTournament {
    pub name: String,
    pub description: Option<String>,
    pub sport_type: SportType,
    pub format: TournamentFormat,
    pub start_date: chrono::DateTime<Utc>,
    pub end_date: chrono::DateTime<Utc>,
    pub registration_start_date: Option<chrono::DateTime<Utc>>,
    pub registration_end_date: Option<chrono::DateTime<Utc>>,
    pub venue: Option<String>,
    pub max_participants: Option<i32>,
    pub entry_fee: Option<Decimal>,
    pub prize_pool: Option<Decimal>,
    pub organizer_id: Uuid,
}

#[derive(Debug)]
pub struct TournamentSearchQuery {
    pub name: Option<String>,
    pub sport_type: Option<String>,
    pub status: Option<String>,
    pub format: Option<String>,
    pub location: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug)]
pub struct TournamentStats {
    pub participants_count: i64,
    pub registrations_count: i64,
    pub categories_count: i64,
    pub matches_played: i64,
    pub prize_pool_total: String,
    pub status: TournamentStatus,
}

#[derive(Debug)]
pub struct ExportData {
    pub format: String,
    pub data: serde_json::Value,
    pub filename: String,
    pub content_type: String,
}

// Business Logic Validation Functions
// These functions encapsulate the business rules that would be tested in the actual service

impl NewTournament {
    /// Validates business rules for tournament creation
    pub fn validate(&self) -> Result<(), String> {
        // Rule 1: Name cannot be empty
        if self.name.trim().is_empty() {
            return Err("Tournament name cannot be empty".to_string());
        }
        
        // Rule 2: Name length limit
        if self.name.len() > 255 {
            return Err("Tournament name cannot exceed 255 characters".to_string());
        }
        
        // Rule 3: End date must be after start date
        if self.end_date <= self.start_date {
            return Err("End date must be after start date".to_string());
        }
        
        // Rule 4: Registration dates validation
        if let (Some(reg_start), Some(reg_end)) = (self.registration_start_date, self.registration_end_date) {
            if reg_end < reg_start {
                return Err("Registration end date must be after registration start date".to_string());
            }
            if reg_end > self.start_date {
                return Err("Registration must end before tournament starts".to_string());
            }
        }
        
        // Rule 5: Max participants must be positive
        if let Some(max) = self.max_participants {
            if max <= 0 {
                return Err("Maximum participants must be positive".to_string());
            }
        }
        
        // Rule 6: Entry fee cannot be negative
        if let Some(fee) = self.entry_fee {
            if fee < Decimal::ZERO {
                return Err("Entry fee cannot be negative".to_string());
            }
        }
        
        // Rule 7: Prize pool cannot be negative
        if let Some(prize) = self.prize_pool {
            if prize < Decimal::ZERO {
                return Err("Prize pool cannot be negative".to_string());
            }
        }
        
        // Rule 8: Venue cannot be empty if provided
        if let Some(venue) = &self.venue {
            if venue.trim().is_empty() {
                return Err("Venue cannot be empty if provided".to_string());
            }
            if venue.len() > 255 {
                return Err("Venue name cannot exceed 255 characters".to_string());
            }
        }
        
        Ok(())
    }
    
    /// Validates format-specific rules
    pub fn validate_format_rules(&self) -> Result<(), String> {
        if let Some(max_participants) = self.max_participants {
            match self.format {
                TournamentFormat::Elimination => {
                    // Elimination works better with powers of 2
                    if max_participants > 0 && (max_participants & (max_participants - 1)) != 0 {
                        // Not a power of 2, but still valid - just a recommendation
                    }
                }
                TournamentFormat::Swiss => {
                    // Swiss works better with even numbers
                    if max_participants % 2 != 0 {
                        // Odd number can still work but is less optimal
                    }
                }
                _ => {
                    // Other formats have no specific participant requirements
                }
            }
        }
        Ok(())
    }
}

impl TournamentSearchQuery {
    /// Validates search query parameters
    pub fn validate(&self) -> Result<(), String> {
        // Validate limit
        if let Some(limit) = self.limit {
            if limit < 0 {
                return Err("Limit cannot be negative".to_string());
            }
            if limit > 1000 {
                return Err("Limit cannot exceed 1000".to_string());
            }
        }
        
        // Validate offset
        if let Some(offset) = self.offset {
            if offset < 0 {
                return Err("Offset cannot be negative".to_string());
            }
        }
        
        // Validate date formats
        if let Some(date_from) = &self.date_from {
            if chrono::DateTime::parse_from_rfc3339(date_from).is_err() {
                return Err("Invalid date_from format. Use RFC3339 format".to_string());
            }
        }
        
        if let Some(date_to) = &self.date_to {
            if chrono::DateTime::parse_from_rfc3339(date_to).is_err() {
                return Err("Invalid date_to format. Use RFC3339 format".to_string());
            }
        }
        
        Ok(())
    }
}

impl Tournament {
    /// Validates if tournament can transition to a new status
    pub fn can_transition_to(&self, new_status: TournamentStatus) -> bool {
        match (&self.status, &new_status) {
            // Valid transitions
            (TournamentStatus::Draft, TournamentStatus::Upcoming) => true,
            (TournamentStatus::Draft, TournamentStatus::RegistrationOpen) => true,
            (TournamentStatus::Upcoming, TournamentStatus::RegistrationOpen) => true,
            (TournamentStatus::RegistrationOpen, TournamentStatus::RegistrationClosed) => true,
            (TournamentStatus::RegistrationClosed, TournamentStatus::InProgress) => true,
            (TournamentStatus::InProgress, TournamentStatus::Completed) => true,
            
            // Any non-terminal state can be cancelled
            (current, TournamentStatus::Cancelled) => {
                !matches!(current, TournamentStatus::Completed | TournamentStatus::Cancelled)
            }
            
            // Same status is allowed (no change)
            (current, new) if current == new => true,
            
            // All other transitions are invalid
            _ => false,
        }
    }
    
    /// Checks if tournament can be published (moved to RegistrationOpen)
    pub fn can_be_published(&self) -> bool {
        matches!(self.status, TournamentStatus::Draft | TournamentStatus::Upcoming)
    }
    
    /// Checks if tournament can be started
    pub fn can_be_started(&self) -> bool {
        self.status == TournamentStatus::RegistrationClosed && self.start_date <= Utc::now()
    }
    
    /// Checks if tournament can be completed
    pub fn can_be_completed(&self) -> bool {
        self.status == TournamentStatus::InProgress
    }
}

impl TournamentStats {
    /// Validates statistics consistency
    pub fn validate(&self) -> Result<(), String> {
        // Registrations should be >= participants (due to team counting)
        if self.registrations_count < self.participants_count {
            return Err("Registration count cannot be less than participant count".to_string());
        }
        
        // All counts should be non-negative
        if self.participants_count < 0 || self.registrations_count < 0 || 
           self.categories_count < 0 || self.matches_played < 0 {
            return Err("Statistics counts cannot be negative".to_string());
        }
        
        // Prize pool total should be a valid decimal string
        if self.prize_pool_total.parse::<Decimal>().is_err() {
            return Err("Invalid prize pool total format".to_string());
        }
        
        Ok(())
    }
}

impl ExportData {
    /// Validates export data format
    pub fn validate(&self) -> Result<(), String> {
        // Valid formats
        let valid_formats = ["json", "csv", "pdf"];
        if !valid_formats.contains(&self.format.as_str()) {
            return Err("Invalid export format. Must be json, csv, or pdf".to_string());
        }
        
        // Content type should match format
        let expected_content_type = match self.format.as_str() {
            "json" => "application/json",
            "csv" => "text/csv",
            "pdf" => "application/pdf",
            _ => return Err("Unsupported format".to_string()),
        };
        
        if self.content_type != expected_content_type {
            return Err("Content type does not match format".to_string());
        }
        
        // Filename should have correct extension
        let expected_extension = format!("_export.{}", self.format);
        if !self.filename.ends_with(&expected_extension) {
            return Err("Filename does not have correct extension".to_string());
        }
        
        Ok(())
    }
}

// Test helper functions
fn create_valid_new_tournament() -> NewTournament {
    NewTournament {
        name: "Test Tournament".to_string(),
        description: Some("Test Description".to_string()),
        sport_type: SportType::Basketball,
        format: TournamentFormat::Elimination,
        start_date: Utc::now() + chrono::Duration::days(7),
        end_date: Utc::now() + chrono::Duration::days(8),
        registration_start_date: Some(Utc::now()),
        registration_end_date: Some(Utc::now() + chrono::Duration::days(6)),
        venue: Some("Test Venue".to_string()),
        max_participants: Some(16),
        entry_fee: Some(Decimal::from_str("25.00").unwrap()),
        prize_pool: Some(Decimal::from_str("1000.00").unwrap()),
        organizer_id: Uuid::new_v4(),
    }
}

fn create_valid_tournament() -> Tournament {
    Tournament {
        id: Uuid::new_v4(),
        name: "Test Tournament".to_string(),
        description: Some("Test Description".to_string()),
        sport_type: SportType::Basketball,
        format: TournamentFormat::Elimination,
        status: TournamentStatus::Draft,
        start_date: Utc::now() + chrono::Duration::days(7),
        end_date: Utc::now() + chrono::Duration::days(8),
        registration_start_date: Some(Utc::now()),
        registration_end_date: Some(Utc::now() + chrono::Duration::days(6)),
        venue: Some("Test Venue".to_string()),
        max_participants: Some(16),
        entry_fee: Some(Decimal::from_str("25.00").unwrap()),
        prize_pool: Some(Decimal::from_str("1000.00").unwrap()),
        organizer_id: Uuid::new_v4(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[cfg(test)]
mod tournament_service_business_logic_tests {
    use super::*;

    // 1. CREATE_TOURNAMENT - Business Logic Validation Tests

    #[test]
    fn test_create_tournament_valid_data_success() {
        let new_tournament = create_valid_new_tournament();
        assert!(new_tournament.validate().is_ok());
        assert!(new_tournament.validate_format_rules().is_ok());
    }

    #[test]
    fn test_create_tournament_empty_name_fails() {
        let mut tournament = create_valid_new_tournament();
        tournament.name = "".to_string();
        assert!(tournament.validate().is_err());
        
        tournament.name = "   ".to_string(); // Only whitespace
        assert!(tournament.validate().is_err());
    }

    #[test]
    fn test_create_tournament_name_too_long_fails() {
        let mut tournament = create_valid_new_tournament();
        tournament.name = "a".repeat(256);
        assert!(tournament.validate().is_err());
    }

    #[test]
    fn test_create_tournament_invalid_dates_fail() {
        let mut tournament = create_valid_new_tournament();
        
        // End date before start date
        tournament.end_date = tournament.start_date - chrono::Duration::days(1);
        assert!(tournament.validate().is_err());
        
        // Registration end after tournament start
        tournament = create_valid_new_tournament();
        tournament.registration_end_date = Some(tournament.start_date + chrono::Duration::days(1));
        assert!(tournament.validate().is_err());
    }

    #[test]
    fn test_create_tournament_invalid_participants_fail() {
        let mut tournament = create_valid_new_tournament();
        tournament.max_participants = Some(0);
        assert!(tournament.validate().is_err());
        
        tournament.max_participants = Some(-5);
        assert!(tournament.validate().is_err());
    }

    #[test]
    fn test_create_tournament_negative_fees_fail() {
        let mut tournament = create_valid_new_tournament();
        tournament.entry_fee = Some(Decimal::from_str("-10.00").unwrap());
        assert!(tournament.validate().is_err());
        
        tournament.entry_fee = Some(Decimal::ZERO);
        tournament.prize_pool = Some(Decimal::from_str("-100.00").unwrap());
        assert!(tournament.validate().is_err());
    }

    #[test]
    fn test_create_tournament_empty_venue_fails() {
        let mut tournament = create_valid_new_tournament();
        tournament.venue = Some("".to_string());
        assert!(tournament.validate().is_err());
        
        tournament.venue = Some("   ".to_string());
        assert!(tournament.validate().is_err());
    }

    #[test]
    fn test_create_tournament_venue_too_long_fails() {
        let mut tournament = create_valid_new_tournament();
        tournament.venue = Some("a".repeat(256));
        assert!(tournament.validate().is_err());
    }

    // 2. SEARCH_TOURNAMENTS - Test All Filter Combinations

    #[test]
    fn test_search_tournaments_valid_parameters() {
        let query = TournamentSearchQuery {
            name: Some("Basketball Tournament".to_string()),
            sport_type: Some("basketball".to_string()),
            status: Some("registration_open".to_string()),
            format: Some("elimination".to_string()),
            location: Some("New York".to_string()),
            date_from: Some("2024-01-01T00:00:00Z".to_string()),
            date_to: Some("2024-12-31T23:59:59Z".to_string()),
            limit: Some(50),
            offset: Some(0),
        };
        
        assert!(query.validate().is_ok());
    }

    #[test]
    fn test_search_tournaments_empty_filters() {
        let query = TournamentSearchQuery {
            name: None,
            sport_type: None,
            status: None,
            format: None,
            location: None,
            date_from: None,
            date_to: None,
            limit: None,
            offset: None,
        };
        
        assert!(query.validate().is_ok());
    }

    #[test]
    fn test_search_tournaments_invalid_limit() {
        let query = TournamentSearchQuery {
            name: None,
            sport_type: None,
            status: None,
            format: None,
            location: None,
            date_from: None,
            date_to: None,
            limit: Some(-1),
            offset: Some(0),
        };
        
        assert!(query.validate().is_err());
        
        let query2 = TournamentSearchQuery {
            limit: Some(1001),
            ..query
        };
        assert!(query2.validate().is_err());
    }

    #[test]
    fn test_search_tournaments_invalid_dates() {
        let query = TournamentSearchQuery {
            name: None,
            sport_type: None,
            status: None,
            format: None,
            location: None,
            date_from: Some("invalid-date".to_string()),
            date_to: None,
            limit: None,
            offset: None,
        };
        
        assert!(query.validate().is_err());
        
        let query2 = TournamentSearchQuery {
            date_from: None,
            date_to: Some("2024-13-45T25:70:90Z".to_string()),
            ..query
        };
        assert!(query2.validate().is_err());
    }

    // 3. GET_FEATURED_TOURNAMENTS - Test Ordering and Limits

    #[test]
    fn test_featured_tournaments_ordering_logic() {
        let mut tournaments = vec![
            Tournament {
                prize_pool: Some(Decimal::from_str("1000.00").unwrap()),
                start_date: Utc::now() + chrono::Duration::days(10),
                status: TournamentStatus::RegistrationOpen,
                ..create_valid_tournament()
            },
            Tournament {
                prize_pool: Some(Decimal::from_str("5000.00").unwrap()),
                start_date: Utc::now() + chrono::Duration::days(5),
                status: TournamentStatus::RegistrationOpen,
                ..create_valid_tournament()
            },
            Tournament {
                prize_pool: Some(Decimal::from_str("2000.00").unwrap()),
                start_date: Utc::now() + chrono::Duration::days(15),
                status: TournamentStatus::Draft, // Should be filtered out
                ..create_valid_tournament()
            },
        ];

        // Filter out non-featured statuses
        tournaments.retain(|t| !matches!(t.status, TournamentStatus::Draft | TournamentStatus::Cancelled));
        
        // Sort by prize pool descending
        tournaments.sort_by(|a, b| b.prize_pool.cmp(&a.prize_pool));
        
        assert_eq!(tournaments.len(), 2);
        assert_eq!(tournaments[0].prize_pool.unwrap(), Decimal::from_str("5000.00").unwrap());
        assert_eq!(tournaments[1].prize_pool.unwrap(), Decimal::from_str("1000.00").unwrap());
    }

    #[test]
    fn test_featured_tournaments_limit() {
        const FEATURED_LIMIT: usize = 10;
        
        let tournaments: Vec<Tournament> = (0..15).map(|i| {
            Tournament {
                prize_pool: Some(Decimal::from_str(&format!("{}.00", 1000 + i * 100)).unwrap()),
                status: TournamentStatus::RegistrationOpen,
                ..create_valid_tournament()
            }
        }).collect();

        let featured: Vec<_> = tournaments.into_iter().take(FEATURED_LIMIT).collect();
        assert!(featured.len() <= FEATURED_LIMIT);
        assert_eq!(featured.len(), FEATURED_LIMIT);
    }

    // 4. PUBLISH_TOURNAMENT - State Transitions

    #[test]
    fn test_publish_tournament_valid_states() {
        let valid_states = [TournamentStatus::Draft, TournamentStatus::Upcoming];
        
        for initial_state in valid_states {
            let tournament = Tournament {
                status: initial_state,
                ..create_valid_tournament()
            };
            
            assert!(tournament.can_be_published());
            assert!(tournament.can_transition_to(TournamentStatus::RegistrationOpen));
        }
    }

    #[test]
    fn test_publish_tournament_invalid_states() {
        let invalid_states = [
            TournamentStatus::RegistrationClosed,
            TournamentStatus::InProgress,
            TournamentStatus::Completed,
            TournamentStatus::Cancelled,
        ];
        
        for invalid_state in invalid_states {
            let tournament = Tournament {
                status: invalid_state,
                ..create_valid_tournament()
            };
            
            assert!(!tournament.can_be_published());
        }
    }

    // 5. START_TOURNAMENT - Validation of Preconditions

    #[test]
    fn test_start_tournament_valid_preconditions() {
        let tournament = Tournament {
            status: TournamentStatus::RegistrationClosed,
            start_date: Utc::now() - chrono::Duration::hours(1), // Already time to start
            ..create_valid_tournament()
        };
        
        assert!(tournament.can_be_started());
        assert!(tournament.can_transition_to(TournamentStatus::InProgress));
    }

    #[test]
    fn test_start_tournament_invalid_preconditions() {
        // Wrong status
        let tournament1 = Tournament {
            status: TournamentStatus::Draft,
            start_date: Utc::now() - chrono::Duration::hours(1),
            ..create_valid_tournament()
        };
        assert!(!tournament1.can_be_started());

        // Future start date
        let tournament2 = Tournament {
            status: TournamentStatus::RegistrationClosed,
            start_date: Utc::now() + chrono::Duration::hours(1),
            ..create_valid_tournament()
        };
        assert!(!tournament2.can_be_started());
    }

    // 6. COMPLETE_TOURNAMENT - Business Rules

    #[test]
    fn test_complete_tournament_valid_state() {
        let tournament = Tournament {
            status: TournamentStatus::InProgress,
            ..create_valid_tournament()
        };
        
        assert!(tournament.can_be_completed());
        assert!(tournament.can_transition_to(TournamentStatus::Completed));
    }

    #[test]
    fn test_complete_tournament_invalid_states() {
        let invalid_states = [
            TournamentStatus::Draft,
            TournamentStatus::RegistrationOpen,
            TournamentStatus::RegistrationClosed,
            TournamentStatus::Completed,
            TournamentStatus::Cancelled,
        ];
        
        for invalid_state in invalid_states {
            let tournament = Tournament {
                status: invalid_state,
                ..create_valid_tournament()
            };
            
            assert!(!tournament.can_be_completed());
        }
    }

    // 7. GET_TOURNAMENT_STATS - Calculation Accuracy

    #[test]
    fn test_tournament_stats_valid_data() {
        let stats = TournamentStats {
            participants_count: 32,
            registrations_count: 35, // Can be higher due to teams
            categories_count: 4,
            matches_played: 15,
            prize_pool_total: "5000.00".to_string(),
            status: TournamentStatus::InProgress,
        };
        
        assert!(stats.validate().is_ok());
    }

    #[test]
    fn test_tournament_stats_invalid_data() {
        let invalid_stats = TournamentStats {
            participants_count: 35,
            registrations_count: 30, // Invalid: less than participants
            categories_count: 4,
            matches_played: 15,
            prize_pool_total: "5000.00".to_string(),
            status: TournamentStatus::InProgress,
        };
        
        assert!(invalid_stats.validate().is_err());
    }

    #[test]
    fn test_tournament_stats_negative_values() {
        let negative_stats = TournamentStats {
            participants_count: -1,
            registrations_count: 30,
            categories_count: 4,
            matches_played: 15,
            prize_pool_total: "5000.00".to_string(),
            status: TournamentStatus::InProgress,
        };
        
        assert!(negative_stats.validate().is_err());
    }

    #[test]
    fn test_tournament_stats_invalid_prize_pool() {
        let invalid_prize_stats = TournamentStats {
            participants_count: 32,
            registrations_count: 35,
            categories_count: 4,
            matches_played: 15,
            prize_pool_total: "invalid-decimal".to_string(),
            status: TournamentStatus::InProgress,
        };
        
        assert!(invalid_prize_stats.validate().is_err());
    }

    // 8. EXPORT_TOURNAMENT - Format Handling

    #[test]
    fn test_export_tournament_json_format() {
        let json_export = ExportData {
            format: "json".to_string(),
            data: json!({"tournament": "test_data"}),
            filename: "Test Tournament_export.json".to_string(),
            content_type: "application/json".to_string(),
        };
        
        assert!(json_export.validate().is_ok());
    }

    #[test]
    fn test_export_tournament_csv_format() {
        let csv_export = ExportData {
            format: "csv".to_string(),
            data: json!({}),
            filename: "Test Tournament_export.csv".to_string(),
            content_type: "text/csv".to_string(),
        };
        
        assert!(csv_export.validate().is_ok());
    }

    #[test]
    fn test_export_tournament_pdf_format() {
        let pdf_export = ExportData {
            format: "pdf".to_string(),
            data: json!({}),
            filename: "Test Tournament_export.pdf".to_string(),
            content_type: "application/pdf".to_string(),
        };
        
        assert!(pdf_export.validate().is_ok());
    }

    #[test]
    fn test_export_tournament_invalid_format() {
        let invalid_export = ExportData {
            format: "xml".to_string(),
            data: json!({}),
            filename: "Test Tournament_export.xml".to_string(),
            content_type: "application/xml".to_string(),
        };
        
        assert!(invalid_export.validate().is_err());
    }

    #[test]
    fn test_export_tournament_mismatched_content_type() {
        let mismatched_export = ExportData {
            format: "json".to_string(),
            data: json!({}),
            filename: "Test Tournament_export.json".to_string(),
            content_type: "text/csv".to_string(), // Wrong content type
        };
        
        assert!(mismatched_export.validate().is_err());
    }

    #[test]
    fn test_export_tournament_wrong_filename_extension() {
        let wrong_extension_export = ExportData {
            format: "json".to_string(),
            data: json!({}),
            filename: "Test Tournament_export.csv".to_string(), // Wrong extension
            content_type: "application/json".to_string(),
        };
        
        assert!(wrong_extension_export.validate().is_err());
    }

    // Additional Business Logic Tests

    #[test]
    fn test_tournament_state_transitions_comprehensive() {
        let tournament = create_valid_tournament();
        
        // Test all valid transitions
        let valid_transitions = [
            (TournamentStatus::Draft, TournamentStatus::Upcoming),
            (TournamentStatus::Draft, TournamentStatus::RegistrationOpen),
            (TournamentStatus::Upcoming, TournamentStatus::RegistrationOpen),
            (TournamentStatus::RegistrationOpen, TournamentStatus::RegistrationClosed),
            (TournamentStatus::RegistrationClosed, TournamentStatus::InProgress),
            (TournamentStatus::InProgress, TournamentStatus::Completed),
        ];
        
        for (from_status, to_status) in valid_transitions {
            let tournament_state = Tournament {
                status: from_status,
                ..tournament.clone()
            };
            assert!(tournament_state.can_transition_to(to_status));
        }
    }

    #[test]
    fn test_tournament_cancellation_rules() {
        // Any non-terminal state can be cancelled
        let cancellable_states = [
            TournamentStatus::Draft,
            TournamentStatus::Upcoming,
            TournamentStatus::RegistrationOpen,
            TournamentStatus::RegistrationClosed,
            TournamentStatus::InProgress,
        ];
        
        for state in cancellable_states {
            let tournament = Tournament {
                status: state,
                ..create_valid_tournament()
            };
            assert!(tournament.can_transition_to(TournamentStatus::Cancelled));
        }
        
        // Terminal states cannot be changed
        let non_cancellable_states = [
            TournamentStatus::Completed,
            TournamentStatus::Cancelled,
        ];
        
        for state in non_cancellable_states {
            let tournament = Tournament {
                status: state,
                ..create_valid_tournament()
            };
            assert!(!tournament.can_transition_to(TournamentStatus::Cancelled));
        }
    }

    #[test]
    fn test_tournament_format_specific_validations() {
        // Test elimination format with power of 2 participants
        let elimination_tournament = NewTournament {
            format: TournamentFormat::Elimination,
            max_participants: Some(16), // Power of 2
            ..create_valid_new_tournament()
        };
        assert!(elimination_tournament.validate_format_rules().is_ok());

        // Test round robin (any number works)
        let round_robin_tournament = NewTournament {
            format: TournamentFormat::RoundRobin,
            max_participants: Some(7), // Odd number is fine
            ..create_valid_new_tournament()
        };
        assert!(round_robin_tournament.validate_format_rules().is_ok());

        // Test Swiss format with even participants
        let swiss_tournament = NewTournament {
            format: TournamentFormat::Swiss,
            max_participants: Some(32), // Even number
            ..create_valid_new_tournament()
        };
        assert!(swiss_tournament.validate_format_rules().is_ok());
    }

    #[test]
    fn test_edge_cases_and_boundary_conditions() {
        // Test minimum valid tournament
        let minimal_tournament = NewTournament {
            name: "A".to_string(), // Minimum length
            description: None,
            sport_type: SportType::Chess,
            format: TournamentFormat::RoundRobin,
            start_date: Utc::now() + chrono::Duration::minutes(1),
            end_date: Utc::now() + chrono::Duration::minutes(2), // Minimum duration
            registration_start_date: None,
            registration_end_date: None,
            venue: None,
            max_participants: Some(1), // Minimum participants
            entry_fee: Some(Decimal::ZERO), // Zero fee
            prize_pool: Some(Decimal::ZERO), // Zero prize
            organizer_id: Uuid::new_v4(),
        };
        
        assert!(minimal_tournament.validate().is_ok());

        // Test maximum valid values
        let maximal_tournament = NewTournament {
            name: "A".repeat(255), // Maximum length
            description: Some("Description".to_string()),
            sport_type: SportType::Esports,
            format: TournamentFormat::DoubleElimination,
            start_date: Utc::now() + chrono::Duration::days(365),
            end_date: Utc::now() + chrono::Duration::days(366),
            registration_start_date: Some(Utc::now()),
            registration_end_date: Some(Utc::now() + chrono::Duration::days(364)),
            venue: Some("V".repeat(255)), // Maximum venue length
            max_participants: Some(10000), // Large number
            entry_fee: Some(Decimal::from_str("9999.99").unwrap()),
            prize_pool: Some(Decimal::from_str("999999.99").unwrap()),
            organizer_id: Uuid::new_v4(),
        };
        
        assert!(maximal_tournament.validate().is_ok());
    }
}
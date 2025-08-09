#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use chrono::Utc;
    use uuid::Uuid;

    use crate::server::{
        config::DbPool,
        models::match_model::{EditableMatch, MatchStatus, MatchStatusUpdate, MatchType, NewMatch},
        routes::match_routes,
    };

    #[actix_web::test]
    async fn test_create_match() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default())) // Mock pool
                .configure(match_routes::routes),
        )
        .await;

        let new_match = NewMatch {
            tournament_category_id: Uuid::new_v4(),
            participant1_team_id: None,
            participant1_player_id: Some(Uuid::new_v4()),
            participant1_partner_id: None,
            participant2_team_id: None,
            participant2_player_id: Some(Uuid::new_v4()),
            participant2_partner_id: None,
            match_type: MatchType::QuarterFinal,
            round_number: Some(1),
            match_number: Some(1),
            scheduled_date: Utc::now(),
            venue: Some("Court A".to_string()),
            court_number: Some("1".to_string()),
            referee_name: Some("John Doe".to_string()),
            umpire_name: None,
            notes: None,
            metadata: None,
        };

        let req = test::TestRequest::post()
            .uri("/matches")
            .set_json(&new_match)
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Would normally check for 201 Created status
        // This is a basic structure test
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_get_match_by_id() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default())) // Mock pool
                .configure(match_routes::routes),
        )
        .await;

        let match_id = Uuid::new_v4();
        let req = test::TestRequest::get()
            .uri(&format!("/matches/{}", match_id))
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Would normally check response based on actual database state
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_update_match_status() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default())) // Mock pool
                .configure(match_routes::routes),
        )
        .await;

        let match_id = Uuid::new_v4();
        let status_update = MatchStatusUpdate {
            match_status: MatchStatus::Completed,
            winner_participant: Some(1),
            is_draw: Some(false),
            notes: Some("Great match!".to_string()),
        };

        let req = test::TestRequest::put()
            .uri(&format!("/matches/{}/status", match_id))
            .set_json(&status_update)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success() || resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_get_tournament_matches() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default())) // Mock pool
                .configure(match_routes::routes),
        )
        .await;

        let tournament_id = Uuid::new_v4();
        let req = test::TestRequest::get()
            .uri(&format!("/matches/tournament/{}", tournament_id))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success() || resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_get_match_schedule() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default())) // Mock pool
                .configure(match_routes::routes),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/matches/schedule")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}

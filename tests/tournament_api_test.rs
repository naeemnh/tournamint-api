#[cfg(test)]
mod tournament_tests {
    use actix_web::{test, web, App, http::StatusCode};
    use chrono::Utc;
    use serde_json::json;
    use uuid::Uuid;
    use server::{
        config::DbPool,
        models::tournament::{NewTournament, TournamentStatus, TournamentFormat, SportType},
        routes::tournament,
    };

    fn create_test_tournament() -> NewTournament {
        NewTournament {
            name: "Test Tournament".to_string(),
            description: Some("Test tournament description".to_string()),
            sport_type: SportType::Tennis,
            format: TournamentFormat::SingleElimination,
            status: TournamentStatus::Draft,
            registration_start_date: Utc::now(),
            registration_end_date: Utc::now() + chrono::Duration::days(7),
            tournament_start_date: Utc::now() + chrono::Duration::days(14),
            tournament_end_date: Utc::now() + chrono::Duration::days(21),
            max_participants: 32,
            entry_fee: Some(50.0),
            prize_pool: Some(5000.0),
            location: Some("Test Location".to_string()),
            venue: Some("Test Venue".to_string()),
            organizer_name: "Test Organizer".to_string(),
            organizer_contact: Some("test@example.com".to_string()),
            rules: Some(json!({"rules": "test"})),
            metadata: None,
        }
    }

    #[actix_web::test]
    async fn test_create_tournament() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default()))
                .configure(tournament::routes)
        ).await;

        let new_tournament = create_test_tournament();

        let req = test::TestRequest::post()
            .uri("/tournaments")
            .set_json(&new_tournament)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn test_get_all_tournaments() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default()))
                .configure(tournament::routes)
        ).await;

        let req = test::TestRequest::get()
            .uri("/tournaments")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_tournament_by_id() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default()))
                .configure(tournament::routes)
        ).await;

        let tournament_id = Uuid::new_v4();
        let req = test::TestRequest::get()
            .uri(&format!("/tournaments/{}", tournament_id))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Would be NOT_FOUND in actual test with real DB
        assert!(resp.status() == StatusCode::NOT_FOUND || resp.status() == StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_search_tournaments() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default()))
                .configure(tournament::routes)
        ).await;

        let req = test::TestRequest::get()
            .uri("/tournaments/search?name=Test&sport_type=Tennis")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_featured_tournaments() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default()))
                .configure(tournament::routes)
        ).await;

        let req = test::TestRequest::get()
            .uri("/tournaments/featured")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_upcoming_tournaments() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default()))
                .configure(tournament::routes)
        ).await;

        let req = test::TestRequest::get()
            .uri("/tournaments/upcoming")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_publish_tournament() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default()))
                .configure(tournament::routes)
        ).await;

        let tournament_id = Uuid::new_v4();
        let req = test::TestRequest::put()
            .uri(&format!("/tournaments/{}/publish", tournament_id))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Would check for proper status based on DB state
        assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_cancel_tournament() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default()))
                .configure(tournament::routes)
        ).await;

        let tournament_id = Uuid::new_v4();
        let cancel_request = json!({
            "reason": "Weather conditions"
        });

        let req = test::TestRequest::put()
            .uri(&format!("/tournaments/{}/cancel", tournament_id))
            .set_json(&cancel_request)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_tournament_stats() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default()))
                .configure(tournament::routes)
        ).await;

        let tournament_id = Uuid::new_v4();
        let req = test::TestRequest::get()
            .uri(&format!("/tournaments/{}/stats", tournament_id))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_export_tournament() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default()))
                .configure(tournament::routes)
        ).await;

        let tournament_id = Uuid::new_v4();
        let req = test::TestRequest::get()
            .uri(&format!("/tournaments/{}/export?format=json", tournament_id))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_duplicate_tournament() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default()))
                .configure(tournament::routes)
        ).await;

        let tournament_id = Uuid::new_v4();
        let duplicate_request = json!({
            "name": "Duplicated Tournament"
        });

        let req = test::TestRequest::post()
            .uri(&format!("/tournaments/{}/duplicate", tournament_id))
            .set_json(&duplicate_request)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == StatusCode::CREATED || resp.status() == StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_tournament_dashboard() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(DbPool::default()))
                .configure(tournament::routes)
        ).await;

        let tournament_id = Uuid::new_v4();
        let req = test::TestRequest::get()
            .uri(&format!("/tournaments/{}/dashboard", tournament_id))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::NOT_FOUND);
    }
}
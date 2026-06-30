use super::db::{insert_lead, Lead};
use chrono::Utc;
use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationTask {
    pub id: Option<i32>,
    pub dark_signal_id: Option<i32>,
    pub observation_id: Option<i32>,
    pub lead_id: Option<i32>,
    pub draft_id: Option<i32>,
    pub entity_id: Option<i32>,
    pub check_type: String,
    pub title: String,
    pub description: String,
    pub target_label: String,
    pub target_url: Option<String>,
    pub status: String,
    pub effort_level: String,
    pub impact_level: String,
    pub rank_score: f64,
    pub result_summary: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationQueueSnapshot {
    pub tasks: Vec<VerificationTask>,
    pub generated_count: i32,
}

#[derive(Debug)]
struct SignalSeed {
    id: i32,
    observation_id: Option<i32>,
    title: String,
    origin: String,
    risk_level: String,
    rank_score: f64,
    verification_path: String,
    source_status: Option<String>,
    source_url: Option<String>,
}

#[derive(Debug)]
struct EntitySeed {
    id: i32,
    entity_type: String,
    name: String,
}

pub fn generate_verification_tasks(conn: &Connection) -> SqlResult<i32> {
    let now = Utc::now().to_rfc3339();
    let mut inserted = 0;
    for signal in list_signal_seeds(conn)? {
        inserted += insert_task(
            conn,
            &VerificationTask {
                id: None,
                dark_signal_id: Some(signal.id),
                observation_id: signal.observation_id,
                lead_id: None,
                draft_id: None,
                entity_id: None,
                check_type: "source_reachability".to_string(),
                title: format!("Confirm source access: {}", signal.origin),
                description: "Open the original source and confirm the signal is still reachable and has not been deleted, edited beyond recognition, or moved.".to_string(),
                target_label: signal.origin.clone(),
                target_url: signal.source_url.clone(),
                status: if signal.source_status.as_deref() == Some("online") {
                    "auto_checked".to_string()
                } else {
                    "needs_human".to_string()
                },
                effort_level: "low".to_string(),
                impact_level: impact_from_risk(&signal.risk_level),
                rank_score: task_score(signal.rank_score, "low", &signal.risk_level),
                result_summary: if signal.source_status.as_deref() == Some("online") {
                    Some("Source was online during the most recent fetch.".to_string())
                } else {
                    Some("Source needs a manual reachability check.".to_string())
                },
                created_at: now.clone(),
                updated_at: now.clone(),
            },
        )?;

        inserted += insert_task(
            conn,
            &VerificationTask {
                id: None,
                dark_signal_id: Some(signal.id),
                observation_id: signal.observation_id,
                lead_id: None,
                draft_id: None,
                entity_id: None,
                check_type: "official_record_match".to_string(),
                title: "Find the official-record match".to_string(),
                description: signal.verification_path.clone(),
                target_label: "official records".to_string(),
                target_url: None,
                status: "needs_human".to_string(),
                effort_level: "medium".to_string(),
                impact_level: impact_from_risk(&signal.risk_level),
                rank_score: task_score(signal.rank_score, "medium", &signal.risk_level),
                result_summary: None,
                created_at: now.clone(),
                updated_at: now.clone(),
            },
        )?;

        for entity in entities_for_signal(conn, signal.observation_id)? {
            inserted += insert_task(
                conn,
                &VerificationTask {
                    id: None,
                    dark_signal_id: Some(signal.id),
                    observation_id: signal.observation_id,
                    lead_id: None,
                    draft_id: None,
                    entity_id: Some(entity.id),
                    check_type: "entity_lookup".to_string(),
                    title: format!("Verify {}: {}", entity.entity_type, entity.name),
                    description: format!(
                        "Look up this {} in official or authoritative records, then note whether it confirms, contradicts, or reframes the signal.",
                        entity.entity_type
                    ),
                    target_label: format!("{}: {}", entity.entity_type, entity.name),
                    target_url: None,
                    status: "suggested".to_string(),
                    effort_level: "medium".to_string(),
                    impact_level: impact_from_risk(&signal.risk_level),
                    rank_score: task_score(signal.rank_score, "medium", &signal.risk_level),
                    result_summary: None,
                    created_at: now.clone(),
                    updated_at: now.clone(),
                },
            )?;
        }

        inserted += insert_task(
            conn,
            &VerificationTask {
                id: None,
                dark_signal_id: Some(signal.id),
                observation_id: signal.observation_id,
                lead_id: None,
                draft_id: None,
                entity_id: None,
                check_type: "story_decision".to_string(),
                title: format!("Decide next editorial step: {}", signal.title),
                description: "After checks are complete, decide whether this becomes a story lead, stays monitored, needs more reporting, or should be marked resolved.".to_string(),
                target_label: "editor decision".to_string(),
                target_url: None,
                status: "suggested".to_string(),
                effort_level: "low".to_string(),
                impact_level: impact_from_risk(&signal.risk_level),
                rank_score: task_score(signal.rank_score, "low", &signal.risk_level),
                result_summary: None,
                created_at: now.clone(),
                updated_at: now.clone(),
            },
        )?;
    }
    Ok(inserted)
}

pub fn verification_queue_snapshot(conn: &Connection) -> SqlResult<VerificationQueueSnapshot> {
    let generated_count = generate_verification_tasks(conn)?;
    Ok(VerificationQueueSnapshot {
        tasks: list_verification_tasks(conn)?,
        generated_count,
    })
}

pub fn list_verification_tasks(conn: &Connection) -> SqlResult<Vec<VerificationTask>> {
    let mut stmt = conn.prepare(
        "SELECT id, dark_signal_id, observation_id, lead_id, draft_id, entity_id, check_type, title,
                description, target_label, target_url, status, effort_level, impact_level, rank_score,
                result_summary, created_at, updated_at
         FROM verification_tasks
         ORDER BY
            CASE status
                WHEN 'needs_human' THEN 0
                WHEN 'suggested' THEN 1
                WHEN 'blocked' THEN 2
                WHEN 'auto_checked' THEN 3
                WHEN 'resolved' THEN 4
                ELSE 5
            END,
            rank_score DESC,
            created_at DESC",
    )?;
    let rows = stmt.query_map([], task_from_row)?;
    rows.collect()
}

pub fn update_task_status(
    conn: &Connection,
    id: i32,
    status: &str,
    result_summary: Option<String>,
) -> SqlResult<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE verification_tasks
         SET status = ?1,
             result_summary = COALESCE(?2, result_summary),
             updated_at = ?3
         WHERE id = ?4",
        params![status, result_summary, now, id],
    )?;
    Ok(())
}

pub fn create_lead_from_dark_signal(conn: &Connection, dark_signal_id: i32) -> SqlResult<i32> {
    if let Some(existing_id) = existing_lead_for_signal(conn, dark_signal_id)? {
        return Ok(existing_id);
    }

    let (title, summary, risk_level, observation_id, evidence_id): (
        String,
        String,
        String,
        Option<i32>,
        Option<i32>,
    ) = conn.query_row(
        "SELECT ds.title, ds.summary, ds.risk_level, ds.observation_id, co.evidence_id
         FROM dark_signals ds
         LEFT JOIN civic_observations co ON co.id = ds.observation_id
         WHERE ds.id = ?1",
        params![dark_signal_id],
        |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        },
    )?;

    let checklist = r#"["Confirm the original source is reachable","Find matching official records","Verify named people, companies, parcels, addresses, vendors, and agencies","Decide whether the signal is publishable, monitor-only, or resolved"]"#;
    let lead = Lead {
        id: None,
        detector_name: "dark_signal_desk".to_string(),
        why: format!("{} - {}", title, summary),
        confidence: "low".to_string(),
        risk_level: match risk_level.as_str() {
            "high" => "high",
            "medium" => "med",
            _ => "low",
        }
        .to_string(),
        confirmation_checklist: checklist.to_string(),
        from_scan_lead_id: None,
        story_type: Some("verification".to_string()),
        disposition: Some("needs_verification".to_string()),
        novelty_score: None,
        novelty_reason: Some(
            "Created from a verification task rather than a current-story scan.".to_string(),
        ),
        created_at: Utc::now().to_rfc3339(),
    };
    let evidence_ids = evidence_id.into_iter().collect::<Vec<_>>();
    let lead_id = insert_lead(conn, &lead, &evidence_ids)?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE verification_tasks
         SET lead_id = ?1,
             updated_at = ?2
         WHERE dark_signal_id = ?3",
        params![lead_id, now, dark_signal_id],
    )?;
    conn.execute(
        "UPDATE dark_signals
         SET publication_status = 'verifying',
             updated_at = ?1
         WHERE id = ?2 AND publication_status = 'review'",
        params![now, dark_signal_id],
    )?;
    if let Some(observation_id) = observation_id {
        conn.execute(
            "UPDATE verification_tasks
             SET observation_id = COALESCE(observation_id, ?1)
             WHERE dark_signal_id = ?2",
            params![observation_id, dark_signal_id],
        )?;
    }
    Ok(lead_id)
}

fn insert_task(conn: &Connection, task: &VerificationTask) -> SqlResult<i32> {
    conn.execute(
        "INSERT OR IGNORE INTO verification_tasks (
            dark_signal_id, observation_id, lead_id, draft_id, entity_id, check_type, title,
            description, target_label, target_url, status, effort_level, impact_level, rank_score,
            result_summary, created_at, updated_at
         )
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
        params![
            task.dark_signal_id,
            task.observation_id,
            task.lead_id,
            task.draft_id,
            task.entity_id,
            task.check_type,
            task.title,
            task.description,
            task.target_label,
            task.target_url,
            task.status,
            task.effort_level,
            task.impact_level,
            task.rank_score,
            task.result_summary,
            task.created_at,
            task.updated_at
        ],
    )?;
    Ok(conn.changes() as i32)
}

fn existing_lead_for_signal(conn: &Connection, dark_signal_id: i32) -> SqlResult<Option<i32>> {
    let mut stmt = conn.prepare(
        "SELECT lead_id FROM verification_tasks
         WHERE dark_signal_id = ?1 AND lead_id IS NOT NULL
         ORDER BY updated_at DESC, id DESC
         LIMIT 1",
    )?;
    let mut rows = stmt.query(params![dark_signal_id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

fn list_signal_seeds(conn: &Connection) -> SqlResult<Vec<SignalSeed>> {
    let mut stmt = conn.prepare(
        "SELECT ds.id, ds.observation_id, ds.title, ds.origin, ds.risk_level, ds.rank_score,
                ds.verification_path, s.status, COALESCE(co.url, s.url)
         FROM dark_signals ds
         LEFT JOIN sources s ON s.id = ds.source_id
         LEFT JOIN civic_observations co ON co.id = ds.observation_id
         WHERE ds.publication_status IN ('review', 'verifying', 'ready_for_story')
         ORDER BY ds.rank_score DESC, ds.created_at DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(SignalSeed {
            id: row.get(0)?,
            observation_id: row.get(1)?,
            title: row.get(2)?,
            origin: row.get(3)?,
            risk_level: row.get(4)?,
            rank_score: row.get(5)?,
            verification_path: row.get(6)?,
            source_status: row.get(7)?,
            source_url: row.get(8)?,
        })
    })?;
    rows.collect()
}

fn entities_for_signal(
    conn: &Connection,
    observation_id: Option<i32>,
) -> SqlResult<Vec<EntitySeed>> {
    let Some(observation_id) = observation_id else {
        return Ok(Vec::new());
    };
    let mut stmt = conn.prepare(
        "SELECT e.id, e.entity_type, e.name
         FROM civic_entities e
         JOIN civic_observation_entities oe ON oe.entity_id = e.id
         WHERE oe.observation_id = ?1
         ORDER BY e.entity_type, e.name",
    )?;
    let rows = stmt.query_map(params![observation_id], |row| {
        Ok(EntitySeed {
            id: row.get(0)?,
            entity_type: row.get(1)?,
            name: row.get(2)?,
        })
    })?;
    rows.collect()
}

fn impact_from_risk(risk: &str) -> String {
    match risk {
        "high" => "high",
        "medium" => "medium",
        _ => "low",
    }
    .to_string()
}

fn task_score(signal_score: f64, effort: &str, risk: &str) -> f64 {
    let effort_bonus = match effort {
        "low" => 10.0,
        "medium" => 4.0,
        _ => 0.0,
    };
    let risk_bonus = match risk {
        "high" => 8.0,
        "medium" => 3.0,
        _ => 0.0,
    };
    (signal_score + effort_bonus + risk_bonus).min(100.0)
}

fn task_from_row(row: &rusqlite::Row<'_>) -> SqlResult<VerificationTask> {
    Ok(VerificationTask {
        id: Some(row.get(0)?),
        dark_signal_id: row.get(1)?,
        observation_id: row.get(2)?,
        lead_id: row.get(3)?,
        draft_id: row.get(4)?,
        entity_id: row.get(5)?,
        check_type: row.get(6)?,
        title: row.get(7)?,
        description: row.get(8)?,
        target_label: row.get(9)?,
        target_url: row.get(10)?,
        status: row.get(11)?,
        effort_level: row.get(12)?,
        impact_level: row.get(13)?,
        rank_score: row.get(14)?,
        result_summary: row.get(15)?,
        created_at: row.get(16)?,
        updated_at: row.get(17)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::db::{insert_evidence_item, insert_source, EvidenceItem, Source};
    use crate::core::intelligence::{list_dark_signals, record_evidence_intelligence};
    use rusqlite::Connection;

    #[test]
    fn generates_ranked_tasks_from_dark_signals_once() {
        let mut conn = Connection::open_in_memory().unwrap();
        crate::core::migrations::run_migrations(&mut conn).unwrap();
        let source = Source {
            id: Some(
                insert_source(
                    &conn,
                    &Source {
                        id: None,
                        name: "Town forum".to_string(),
                        url: "https://forum.example.test/thread".to_string(),
                        r#type: "community_signal".to_string(),
                        status: "online".to_string(),
                        tier: "community_signal".to_string(),
                        last_success_at: None,
                        last_failed_at: None,
                        last_scraped: None,
                    },
                )
                .unwrap(),
            ),
            name: "Town forum".to_string(),
            url: "https://forum.example.test/thread".to_string(),
            r#type: "community_signal".to_string(),
            status: "online".to_string(),
            tier: "community_signal".to_string(),
            last_success_at: None,
            last_failed_at: None,
            last_scraped: None,
        };
        let item = EvidenceItem {
            id: None,
            source_id: source.id.unwrap(),
            url: Some(source.url.clone()),
            fetched_at: Utc::now().to_rfc3339(),
            excerpt: "Residents say Acme Development LLC quietly bought parcel APN 123-456-789."
                .to_string(),
            content_hash: "hash-verification".to_string(),
            entities: "[]".to_string(),
        };
        let evidence_id = insert_evidence_item(&conn, &item).unwrap();
        record_evidence_intelligence(&conn, &source, &item, evidence_id, None).unwrap();
        assert_eq!(list_dark_signals(&conn, 10).unwrap().len(), 1);

        let first = generate_verification_tasks(&conn).unwrap();
        let second = generate_verification_tasks(&conn).unwrap();
        let tasks = list_verification_tasks(&conn).unwrap();

        assert!(first >= 4);
        assert_eq!(second, 0);
        assert!(tasks
            .iter()
            .any(|t| t.check_type == "official_record_match"));
        assert!(tasks.iter().any(|t| t.check_type == "entity_lookup"));
        assert!(tasks.iter().any(|t| t.status == "auto_checked"));
        assert!(tasks.iter().all(|t| t.dark_signal_id.is_some()));
    }

    #[test]
    fn updates_task_status_without_deleting_context() {
        let mut conn = Connection::open_in_memory().unwrap();
        crate::core::migrations::run_migrations(&mut conn).unwrap();
        conn.execute(
            "INSERT INTO verification_tasks (check_type, title, description, target_label, status, effort_level, impact_level, rank_score, created_at, updated_at)
             VALUES ('evidence_gap', 'Check packet', 'Find missing packet', 'packet', 'suggested', 'low', 'medium', 20, 'now', 'now')",
            [],
        )
        .unwrap();
        update_task_status(&conn, 1, "resolved", Some("Packet found.".to_string())).unwrap();

        let tasks = list_verification_tasks(&conn).unwrap();
        assert_eq!(tasks[0].status, "resolved");
        assert_eq!(tasks[0].result_summary.as_deref(), Some("Packet found."));
        assert_eq!(tasks[0].title, "Check packet");
    }

    #[test]
    fn creates_story_lead_from_dark_signal_and_links_tasks() {
        let mut conn = Connection::open_in_memory().unwrap();
        crate::core::migrations::run_migrations(&mut conn).unwrap();
        let source = Source {
            id: Some(
                insert_source(
                    &conn,
                    &Source {
                        id: None,
                        name: "Town forum".to_string(),
                        url: "https://forum.example.test/thread".to_string(),
                        r#type: "community_signal".to_string(),
                        status: "online".to_string(),
                        tier: "community_signal".to_string(),
                        last_success_at: None,
                        last_failed_at: None,
                        last_scraped: None,
                    },
                )
                .unwrap(),
            ),
            name: "Town forum".to_string(),
            url: "https://forum.example.test/thread".to_string(),
            r#type: "community_signal".to_string(),
            status: "online".to_string(),
            tier: "community_signal".to_string(),
            last_success_at: None,
            last_failed_at: None,
            last_scraped: None,
        };
        let item = EvidenceItem {
            id: None,
            source_id: source.id.unwrap(),
            url: Some(source.url.clone()),
            fetched_at: Utc::now().to_rfc3339(),
            excerpt: "Residents say Acme Development LLC quietly bought parcel APN 123-456-789."
                .to_string(),
            content_hash: "hash-lead-bridge".to_string(),
            entities: "[]".to_string(),
        };
        let evidence_id = insert_evidence_item(&conn, &item).unwrap();
        record_evidence_intelligence(&conn, &source, &item, evidence_id, None).unwrap();
        let signal_id = list_dark_signals(&conn, 10).unwrap()[0].id.unwrap();
        generate_verification_tasks(&conn).unwrap();

        let lead_id = create_lead_from_dark_signal(&conn, signal_id).unwrap();
        let second_id = create_lead_from_dark_signal(&conn, signal_id).unwrap();
        let tasks = list_verification_tasks(&conn).unwrap();

        assert_eq!(lead_id, second_id);
        assert!(tasks.iter().all(|task| task.lead_id == Some(lead_id)));
        assert_eq!(
            crate::core::db::get_evidence_by_lead(&conn, lead_id)
                .unwrap()
                .len(),
            1
        );
    }
}

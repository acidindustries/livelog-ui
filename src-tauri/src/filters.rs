use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Sqlite};
use super::models::Log;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    color: Option<String>,
    level: Option<String>,
    search: Option<String>,
}

impl Filter {
    pub fn new(color: Option<String>, level: Option<String>, search: Option<String>) -> Self {
        Filter {
            color,
            level,
            search
        }
    }

    pub fn get_query<'a>(self, qb: &'a mut QueryBuilder<'a, Sqlite>) -> sqlx::query::QueryAs<'_, Sqlite, Log, sqlx::sqlite::SqliteArguments<'_>> {
        let mut some_search = false;
        let mut some_color = false;
        if self.search.is_some() || self.color.is_some() || self.level.is_some() {
            qb.push(" WHERE");
            if self.search.is_some() {
                some_search = true;
            }

            if self.search.is_some() {
                some_color = true;
            }
        }

        if let Some(search) = &self.search {
            if !search.is_empty() {
                qb
                    .push(r" (json_extract(logs.data, '$.data', '$.message') LIKE ")
                    .push_bind(format!("%{}%", search.clone()))
                    .push(r" OR json_extract(logs.source, '$.file', '$.line') LIKE ")
                    .push_bind(format!("%{}%", search))
                    .push(")");
            }
            else {
                some_search = false;
            }
        }

        if let Some(colors) = &self.color {
            let mut nullable = false;
            if some_search {
                qb.push(" AND");
            }
            let colors = colors
                            .split(",")
                            .filter(|color| !color.is_empty())
                            .collect::<Vec<&str>>();

            if colors.contains(&"null") {
                nullable = true;
            }

            let colors = colors
                            .iter()
                            .map(|color| format!("'{}'", color))
                            .collect::<Vec<String>>()
                            .join(",");

            qb.push(" (color IN (");
            qb.push(colors.clone());
            println!("{}", colors);
            qb.push(")");
            if nullable {
                if colors.len() > 0 {
                    qb.push(" OR");
                }
                qb.push(" color IS NULL");
            }
            qb.push(")");
        }

        if let Some(levels) = &self.level {
            if some_color {
                qb.push(" AND");
            }
            let levels = levels
                            .split(",")
                            .filter(|level| !level.is_empty())
                            .collect::<Vec<&str>>();

            let levels = levels
                            .iter()
                            .map(|level| format!("'{}'", level))
                            .collect::<Vec<String>>()
                            .join(",");

            qb.push(" (level IN (");
            qb.push(levels.clone());
            println!("{}", levels);
            qb.push("))");
        }
        qb.push(" ORDER BY date DESC");
        println!("{}", qb.sql());
        qb.build_query_as::<Log>()
    }
}

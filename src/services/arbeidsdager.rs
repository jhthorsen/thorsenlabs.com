use super::helpers::*;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Weekday};
use reqwest;

#[derive(Debug, Deserialize)]
struct HolidaysDocumentDate {
    date: NaiveDateTime,
    description: String,
}

#[derive(Debug, Deserialize)]
struct HolidaysDocument {
    // Ignoring the authenticated, timeTaken, statusCode fields
    data: Vec<HolidaysDocumentDate>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum KnownHolidayKind {
    Holiday,
    Saturday,
    Sunday,
}

#[derive(Debug, Serialize)]
struct KnownHoliday {
    date: NaiveDate,
    description: String,
    kind: KnownHolidayKind,
}

async fn fetch_holidays(year: i32) -> Result<Vec<KnownHoliday>, reqwest::Error> {
    let url = format!("https://webapi.no/api/v1/holidays/{}", year);
    let holidays_doc = reqwest::get(&url).await?.json::<HolidaysDocument>().await?;

    let mut known = holidays_doc
        .data
        .iter()
        .map(|holiday| KnownHoliday {
            kind: KnownHolidayKind::Holiday,
            date: holiday.date.date(),
            description: holiday.description.to_owned(),
        })
        .collect::<Vec<KnownHoliday>>();

    let mut from = NaiveDate::from_yo_opt(year, 1).unwrap_or_default();
    let to = NaiveDate::from_yo_opt(year + 1, 1).unwrap_or_default();
    while from < to {
        if from.weekday() == Weekday::Sat {
            known.push(KnownHoliday {
                kind: KnownHolidayKind::Saturday,
                date: from.clone(),
                description: "Saturday".to_owned(),
            });
        } else if from.weekday() == Weekday::Sun {
            known.push(KnownHoliday {
                kind: KnownHolidayKind::Sunday,
                date: from.clone(),
                description: "Sunday".to_owned(),
            });
        }

        from += Duration::days(1);
    }

    known.sort_by(|a, b| a.date.cmp(&b.date));

    return Ok(known);
}

pub async fn get_arbeidsdager_table(
    State(state): State<crate::AppState>,
    Path(year): Path<i32>,
    headers: HeaderMap,
    uri: Uri,
    method: Method,
) -> Result<Response, ServerError> {
    let mut ctx = crate::template::template_context(&headers, &uri);

    let rendered = if method == Method::HEAD {
        "".to_owned()
    } else {
        ctx.insert("holidays".to_owned(), &fetch_holidays(year).await?);
        state.tera.render("arbeidsdager/table.html", &ctx)?
    };

    Ok((
        StatusCode::OK,
        [
            ("content-type", "text/html"),
            ("cache-control", "max-age=300"),
        ],
        rendered,
    )
        .into_response())
}

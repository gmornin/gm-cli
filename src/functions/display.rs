use std::error::Error;

use chrono::{TimeZone, Utc};
use goodmorning_bindings::structs::{
    BirthDayDetail, CakeDayDetail, ProfileAccount, ProfileCustomisable, ProfileDetail,
};
use log::*;

use crate::error::Error as CError;

pub fn display_profile(
    profile: &ProfileCustomisable,
    account: &ProfileAccount,
    instance: &str,
) -> String {
    let verified = if account.verified {
        ""
    } else {
        "[Not verified]"
    };
    let username = &account.username;
    let joined = Utc
        .timestamp_opt(account.created as i64, 0)
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    let pfp = format!(
        "https://{instance}/api/tex/generic/v1/pfp/id/{}",
        account.id
    );
    let description = if profile.description.is_empty() {
        "\x1B[38;5;8m[empty string]\x1B[0m"
    } else {
        &profile.description
    };
    let details = display_details(&profile.details);

    format!(
        r#"{verified}{username} (joined {joined})

Profile image: `{pfp}`

Description:
{description}

Details
{details}
"#
    )
}

pub fn display_profile_only(profile: &ProfileCustomisable, id: i64, instance: &str) -> String {
    let pfp = format!("https://{instance}/api/tex/generic/v1/pfp/id/{}", id);
    let description = if profile.description.is_empty() {
        "\x1B[38;5;8m[empty string]\x1B[0m"
    } else {
        &profile.description
    };
    let details = display_details(&profile.details);

    format!(
        r#"
Profile image: `{pfp}`

Description:
{description}

Details
{details}
"#
    )
}

pub fn display_details(details: &[ProfileDetail]) -> String {
    if details.is_empty() {
        return "\x1B[38;5;8m[empty list]\x1B[0m".to_string();
    }
    details
        .iter()
        .enumerate()
        .map(|(i, detail)| format!("{}: {}", i + 1, details_to_string(detail)))
        .collect::<Vec<_>>()
        .join("\n")
}

const DETAILS: [&str; 7] = [
    "cake day",
    "birthday",
    "location",
    "occupation",
    "company",
    "school",
    "education level",
];
const DETAILS_PROMPTS: [&str; 7] = [
    "cake day in format `day/month`, for exmaple `20/7`",
    "birthday in format `day/month/year`, for example 20/7/2027",
    "location name",
    "occupation title",
    "company name",
    "school name",
    "education level",
];

pub fn details_list() -> String {
    DETAILS
        .into_iter()
        .enumerate()
        .map(|(i, label)| format!("{}: {label}", i + 1))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn details_prompt(index: usize) -> Result<String, Box<dyn Error>> {
    if index > 7 || index == 0 {
        error!("{index} is not a valid option");
        return Err(CError::StrErr("invalid option").into());
    }

    Ok(format!("Enter your {}", DETAILS_PROMPTS[index-1]))
}

pub fn details_to_string(detail: &ProfileDetail) -> String {
    match detail {
        ProfileDetail::CakeDay {
            value: CakeDayDetail { day, month },
        } => format!("cake day: {day}/{month}"),
        ProfileDetail::BirthDay {
            value: BirthDayDetail { day, month, year },
        } => format!("birthday: {day}/{month}/{year}"),
        ProfileDetail::Location { value } => format!("locaton: {value}"),
        ProfileDetail::Occupation { value } => format!("occupation: {value}"),
        ProfileDetail::Company { value } => format!("company: {value}"),
        ProfileDetail::School { value } => format!("school: {value}"),
        ProfileDetail::EducationLevel { value } => format!("education level: {value}"),
        ProfileDetail::Contact { .. } => todo!(),
    }
}

pub fn details_from_string(index: usize, value: String) -> Result<ProfileDetail, Box<dyn Error>> {
    if index > 7 || index == 0 {
        error!("{index} is not a valid option");
        return Err(CError::StrErr("invalid option").into());
    }

    Ok(match index {
        1 => {
            let (day, month) = match value.split_once('/') {
                Some(value) => value,
                None => return Err(CError::StrErr("expected pattern `day/month`").into()),
            };

            ProfileDetail::CakeDay {
                value: CakeDayDetail {
                    day: day.parse()?,
                    month: month.parse()?,
                },
            }
        }
        2 => {
            let splitted = value.splitn(3, '/').collect::<Vec<_>>();
            if splitted.len() != 3 {
                return Err(CError::StrErr("expected pattern `day/month/year`").into());
            }

            ProfileDetail::BirthDay {
                value: BirthDayDetail {
                    day: splitted[0].parse()?,
                    month: splitted[1].parse()?,
                    year: splitted[2].parse()?,
                },
            }
        }
        3 => ProfileDetail::Location { value },
        4 => ProfileDetail::Occupation { value },
        5 => ProfileDetail::Company { value },
        6 => ProfileDetail::School { value },
        7 => ProfileDetail::EducationLevel { value },
        _ => unreachable!(),
    })
}

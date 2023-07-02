use std::error::Error;

use chrono::{TimeZone, Utc};
use goodmorning_bindings::structs::{
    BirthDayDetail, CakeDayDetail, ContactDetail, ProfileAccount, ProfileCustomisable,
    ProfileDetail,
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
    let status = &account.status;

    format!(
        r#"{verified}{username} (joined {joined})

Status: {status}

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

    Ok(format!("Enter your {}", DETAILS_PROMPTS[index - 1]))
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
        ProfileDetail::Contact { value } => contacts_to_string(value),
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

const CONTACTS: [&str; 13] = [
    "email",
    "matrix",
    "mastodon",
    "lemmy",
    "github",
    "gitlab",
    "bitbucket",
    "reddit",
    "discord",
    "twitter",
    "youtube",
    "odysee",
    "website",
];
const CONTACTS_PROMPTS: [&str; 13] = [
    "email address (e.g. user@example.com)",
    "matrix user id (e.g. user:example.com)",
    "mastodon user id (e.g. user:example.com)",
    "lemmy user id (e.g. user:example.com)",
    "github username (e.g. user)",
    "gitlab username (e.g. user)",
    "bitbucket username (e.g. user)",
    "reddit username (e.g. user)",
    "discord username (e.g. user)",
    "twitter username (e.g. user)",
    "youtube username (e.g. user)",
    "odysee username (e.g. user:1)",
    "website (e.g. example.com without `https://`)",
];

pub fn contacts_list() -> String {
    CONTACTS
        .into_iter()
        .enumerate()
        .map(|(i, label)| format!("{}: {label}", i + 1))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn contacts_prompt(index: usize) -> Result<String, Box<dyn Error>> {
    if index > 13 || index == 0 {
        error!("{index} is not a valid option");
        return Err(CError::StrErr("invalid option").into());
    }
    Ok(format!("Enter your {}", CONTACTS_PROMPTS[index - 1]))
}

pub fn contacts_to_string(contact: &ContactDetail) -> String {
    let contact_type = match contact {
        ContactDetail::Email { name, instance } => format!("email: {name}@{instance}"),
        ContactDetail::Matrix { name, instance } => format!("matrix: {name}:{instance}"),
        ContactDetail::Mastodon { name, instance } => format!("mastodon: {name}:{instance}"),
        ContactDetail::Lemmy { name, instance } => format!("lemmy: {name}:{instance}"),
        ContactDetail::Github { value } => format!("github: {value}"),
        ContactDetail::Gitlab { value } => format!("gitlab: {value}"),
        ContactDetail::Bitbucket { value } => format!("bitbucket: {value}"),
        ContactDetail::Reddit { value } => format!("reddit: {value}"),
        ContactDetail::Discord { value } => format!("discord: {value}"),
        ContactDetail::Twitter { value } => format!("twitter: {value}"),
        ContactDetail::Youtube { value } => format!("youtube: {value}"),
        ContactDetail::Odysee {
            name,
            discriminator,
        } => format!("odysee: {name}:{discriminator}"),
        ContactDetail::Website { value } => format!("website: {value}"),
    };

    format!(
        "{contact_type}{}",
        contacts_to_url(contact)
            .map(|url| format!(" ({url})"))
            .unwrap_or_default()
    )
}

pub fn contacts_from_string(index: usize, value: String) -> Result<ContactDetail, Box<dyn Error>> {
    if index > 13 || index == 0 {
        error!("{index} is not a valid option");
        return Err(CError::StrErr("invalid option").into());
    }

    Ok(match index {
        1 => {
            let (name, instance) = match value.split_once('@') {
                Some((name, instance)) => (name.to_string(), instance.to_string()),
                None => {
                    return Err(
                        CError::StrErr("invalid format, expected `username@instance.com`").into(),
                    )
                }
            };
            ContactDetail::Email { name, instance }
        }
        2 => {
            let (name, instance) = match value.split_once(':') {
                Some((name, instance)) => (name.to_string(), instance.to_string()),
                None => {
                    return Err(
                        CError::StrErr("invalid format, expected `username@instance.com`").into(),
                    )
                }
            };
            ContactDetail::Matrix { name, instance }
        }
        3 => {
            let (name, instance) = match value.split_once(':') {
                Some((name, instance)) => (name.to_string(), instance.to_string()),
                None => {
                    return Err(
                        CError::StrErr("invalid format, expected `username@instance.com`").into(),
                    )
                }
            };
            ContactDetail::Mastodon { name, instance }
        }
        4 => {
            let (name, instance) = match value.split_once(':') {
                Some((name, instance)) => (name.to_string(), instance.to_string()),
                None => {
                    return Err(
                        CError::StrErr("invalid format, expected `username@instance.com`").into(),
                    )
                }
            };
            ContactDetail::Lemmy { name, instance }
        }
        5 => ContactDetail::Github { value },
        6 => ContactDetail::Gitlab { value },
        7 => ContactDetail::Bitbucket { value },
        8 => ContactDetail::Reddit { value },
        9 => ContactDetail::Discord { value },
        10 => ContactDetail::Twitter { value },
        11 => ContactDetail::Youtube { value },
        12 => {
            let (name, discriminator) = match value.split_once(':') {
                Some((name, instance)) => (name.to_string(), instance.parse()?),
                None => {
                    return Err(
                        CError::StrErr("invalid format, expected `username@instance.com`").into(),
                    )
                }
            };
            ContactDetail::Odysee {
                name,
                discriminator,
            }
        }
        13 => ContactDetail::Website { value },
        _ => unreachable!(),
    })
}

pub fn contacts_to_url(contact: &ContactDetail) -> Option<String> {
    match contact {
        ContactDetail::Email { .. } | ContactDetail::Discord { .. } => None,
        ContactDetail::Matrix { name, instance } => {
            Some(format!("https://matrix.to/#/{name}:{instance}"))
        }
        ContactDetail::Mastodon { name, instance } => Some(format!("https://{instance}/@{name}")),
        ContactDetail::Lemmy { name, instance } => Some(format!("https://{instance}/u/{name}")),
        ContactDetail::Github { value } => Some(format!("https://github.com/{value}")),
        ContactDetail::Gitlab { value } => Some(format!("https://gitlab.com/{value}")),
        ContactDetail::Bitbucket { value } => Some(format!("https://bitbucket.com/{value}")),
        ContactDetail::Reddit { value } => Some(format!("https://reddit.com/u/{value}")),
        ContactDetail::Twitter { value } => Some(format!("https://twitter.com/{value}")),
        ContactDetail::Youtube { value } => Some(format!("https://youtube.com/@{value}")),
        ContactDetail::Odysee {
            name,
            discriminator,
        } => Some(format!("https://odysee.com/@{name}:{discriminator}")),
        ContactDetail::Website { value } => Some(format!("https://{value}")),
    }
}

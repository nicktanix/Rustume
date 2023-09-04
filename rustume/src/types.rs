use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Location {
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Social {
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub twitter: Option<String>,
    pub facebook: Option<String>,
    pub instagram: Option<String>,
    pub youtube: Option<String>,
    pub pinterest: Option<String>,
    pub tumblr: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Contact {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    #[serde(rename = "location")]
    pub location: Location,
    #[serde(rename = "social")]
    pub social: Social,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Preferences {
    #[serde(rename = "locationPreference")]
    pub location_preference: Option<Vec<String>>,
    #[serde(rename = "termPreference")]
    pub term_preference: Option<Vec<String>>,
    #[serde(rename = "positionPreference")]
    pub position_preference: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Information {
    pub name: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "currentJobTitle")]
    pub current_job_title: Option<String>,
    pub message: Option<String>,
    pub photo: Option<String>,
    #[serde(rename = "currentTrajectory")]
    pub current_trajectory: Option<Vec<String>>,
    pub preferences: Preferences,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    pub contact: Contact,
    pub information: Information,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Experience {
    pub company: Option<String>,
    pub title: Option<String>,
    pub location: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub description: Option<String>,
    pub highlights: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Education {
    pub institution: Option<String>,
    pub degree: Option<String>,
    #[serde(rename = "fieldOfStudy")]
    pub field_of_study: Option<String>,
    pub location: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub description: Option<String>,
    pub highlights: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SkillItem {
    #[serde(rename = "skillType")]
    pub skill_type: String,
    #[serde(rename = "skillItems")]
    pub skill_items: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Credential {
    pub name: Option<String>,
    pub organization: Option<String>,
    #[serde(rename = "dateAcquired")]
    pub date_acquired: Option<String>,
    #[serde(rename = "dateExpires")]
    pub date_expires: Option<String>,
    #[serde(rename = "credentialType")]
    pub credential_type: Option<String>,
    pub credential: Option<String>,
    #[serde(rename = "credentialId")]
    pub credential_id: Option<String>,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Resume {
    pub profile: Profile,
    pub experience: Option<Vec<Experience>>,
    pub education: Option<Vec<Education>>,
    pub skills: Option<Vec<SkillItem>>,
    pub credentials: Option<Vec<Credential>>,
}

impl Default for Resume {
    fn default() -> Self {
        Self {
            profile: Profile {
                contact: Contact {
                    email: None,
                    phone: None,
                    website: None,
                    location: Location {
                        city: None,
                        state: None,
                        country: None,
                    },
                    social: Social {
                        linkedin: None,
                        github: None,
                        twitter: None,
                        facebook: None,
                        instagram: None,
                        youtube: None,
                        pinterest: None,
                        tumblr: None,
                    },
                },
                information: Information {
                    name: None,
                    title: None,
                    current_job_title: None,
                    message: None,
                    photo: None,
                    current_trajectory: None,
                    preferences: Preferences {
                        location_preference: None,
                        term_preference: None,
                        position_preference: None,
                    },
                },
            },
            experience: None,
            education: None,
            skills: None,
            credentials: None,
        }
    }
}

trait CamelCase {
    fn to_camel_case(&self) -> String;
}

// Implement the trait for types containing a String field
impl CamelCase for String {
    fn to_camel_case(&self) -> String {
        // Convert the string to camel case logic here
        // For example, you can use the code from the previous example
        let parts: Vec<&str> = self.split(&['_', ' ']).collect();

        let camel_case: String = parts
            .into_iter()
            .enumerate()
            .map(|(i, part)| {
                if i == 0 {
                    part.to_string().to_lowercase()
                } else {
                    let mut chars = part.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                }
            })
            .collect();

        camel_case
    }
}


impl SkillItem {
    // Function to return skill_type in camel case
    pub fn skill_type_camel_case(&self) -> String {
        // Split the skill_type by underscores and spaces
        let parts: Vec<&str> = self.skill_type.split(&['_', ' ']).collect();

        // Capitalize the first letter of each part (except the first one)
        let camel_case: String = parts
            .into_iter()
            .enumerate()
            .map(|(i, part)| {
                if i == 0 {
                    part.to_string().to_lowercase() // Convert the first part to lowercase
                } else {
                    let mut chars = part.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                }
            })
            .collect();

        camel_case
    }
}




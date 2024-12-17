// Ensure all relevant items are marked with `pub` keyword
pub const CHRISTMAS_EMOJIS: [char; 4] = ['🎅', '🤶', '🎄', '🎁'];

pub trait EmailAnonymizer {
    fn anonymize_email(&self) -> String;
}

fn generate_emoji_string(length: usize) -> String {
    CHRISTMAS_EMOJIS.iter().cycle().take(length).collect()
}

impl EmailAnonymizer for str {
    fn anonymize_email(&self) -> String {
        // Split the email into local and domain parts
        if let Some((local, domain)) = self.split_once('@') {
            // Generate anonymized local part
            let anonymized_local = generate_emoji_string(local.len());
            format!("{}@{}", anonymized_local, domain)
        } else {
            // Handle invalid emails - replace the entire string
            generate_emoji_string(self.len())
        }
    }
}

pub fn main() {
    let emails = vec![
        "rudolph.therapysessions@northpole.com".to_string(),
        "elfhr.complaint@northpole.urgent".to_string(),
        "santas.rage.management@christmaschaos.noel".to_string(),
        "overtimepay.never@elfexploitation.workshop".to_string(),
        "mrs.claus.divorce.lawyer@northpole.legal".to_string(),
        "reindeer.workers.comp@antler.insurance".to_string(),
        "naughty.list.revenge@santasecrets.com".to_string(),
        "workshop.ptsd.support@elves.anonymous".to_string(),
        "performance.anxiety@santa.breakdown".to_string(),
        "existential.crisis@northpole.void".to_string(),
    ];

    for email in emails {
        let anonymized_email = email.anonymize_email(); // This is the API that Santa wants!
        println!("Original: {} -> Anonymized: {}", email, anonymized_email);
    }
}

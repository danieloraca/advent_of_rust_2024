use std::num::Wrapping;
use std::time::{SystemTime, UNIX_EPOCH};
const CHRISTMAS_EMOJIS: [char; 4] = ['🎅', '🤶', '🎄', '🎁'];
struct EmojiPicker<'a> {
    emojis: &'a [char],
}
impl EmojiPicker<'_> {
    pub fn new() -> Self {
        EmojiPicker {
            emojis: &CHRISTMAS_EMOJIS[..],
        }
    }
    fn random_select(&self) -> char {
        let start = SystemTime::now();
        let duration = start.duration_since(UNIX_EPOCH).unwrap();
        let seed = duration.as_millis() as u32; // Use milliseconds as the seed
        let mut rng = Wrapping(seed);
        const A: u32 = 1664525;
        const C: u32 = 1013904223;
        rng = rng * Wrapping(A) + Wrapping(C);
        let index = (rng.0 % self.emojis.len() as u32) as usize;
        self.emojis[index]
    }
    pub fn christmas_gibberish(&self, len: usize) -> String {
        let mut ret = String::new();
        for _ in 0..len {
            ret.push(self.random_select());
        }
        ret
    }
}
pub trait AnonymizedEmail {
    fn anonymize_email(&self) -> String;
}
impl AnonymizedEmail for String {
    fn anonymize_email(&self) -> String {
        let picker = EmojiPicker::new();
        if let Some((local, domain)) = self.split_once('@') {
            let mut ret = String::new();
            ret.push_str(&picker.christmas_gibberish(local.len()));
            ret.push('@');
            ret.push_str(domain);
            ret
        } else {
            picker.christmas_gibberish(self.len())
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

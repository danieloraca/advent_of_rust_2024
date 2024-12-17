Day 17: Santa's Email Meltdown

“BLITZEN! GET IN HERE!” Santa’s furious voice echoed through the workshop.

Blitzen stepped inside cautiously. It had been only a few hours since Santa forgave him for the "great grep fiasco"—when Blitzen had decided to re-write grep from scratch.

“I thought we were good now!” Blitzen said.

“Well, WE’RE NOT!” Santa shouted, spinning his monitor around. “LOOK AT THIS!”

Blitzen squinted at Santa’s inbox, now overflowing with spam emails:

“Naughty List Revenge Services!”
“SantaCoin: The next big blockchain sleigh innovation!”
“Elves for hire: Cheaper than your workshop!”
“What happened?” Blitzen asked.

“I LEAKED MY EMAIL ON TWITCH!” Santa bellowed. “I was streaming my lecture on why Rust traits are better than cookies when I accidentally typed my real address live on stream!”

The chat, of course, had gone wild:

Chat: “LMAO! Bro just doxxed himself live.” “The spam bots are already in his inbox.”

“And it's all because of YOU!” Santa continued.

“ME? How is this my fault?” Blitzen asked, bewildered.

“If you hadn’t wasted the morning re-writing grep, you’d have caught this issue before it happened!” Santa snapped, slamming his candy cane onto the desk. “Now you’re going to fix it. Write me an API that anonymizes email addresses—Christmas style. Replace the local part with festive emojis, and make sure it doesn’t crash on invalid emails. Do it NOW!”

Blitzen sighed and opened Vim. “Okay, okay… I’m on it.”

Your Mission
Blitzen as always is in trouble—again.

Here's what you gotta do to help him out:

Replace the local part of an email with Christmas emojis, for example santa@north.pole should be anonymized to 🎅🎄🎁🎄🎅@north.pole.
If the email is invalid, turn every character into emojis, for example santa should be anonymized to 🎅🎄🎁🎄🎅.
Here's how Santa likes to use this API:

fn main() {
    let email = "santa@north.pole";
    let anonymized = email.anonymize_email();
}
Figure out a way to make this work, otherwise Blitzen will not get his cookies this Christmas!

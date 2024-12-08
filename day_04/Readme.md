Day 4: Structifying the Naughty List

Santa burst into the dev lounge, chugging his third espresso. "Great job yesterday, nerds! The is_nice function? Chef's kiss. But now, I want structure. STRUCTURE! We’re going full-on Rustacean. I need a Kid struct—immediately!"

The elves nodded enthusiastically, their tiny laptops open, running Arch Linux with bspwm because, obviously, they were that kind of devs. One elf, started yapping, "But Santa, why a struct? Isn’t this just overengineered?"

Santa slammed the table, shaking an untouched tray of gluten-free cookies. "No! A struct means no more random strings floating around. We need to encapsulate a kid's data—name, and niceness score. Plus, we’ll need some methods to make sense of it all."

The dev elves scrambled to work. In no time, they sketched out the basic blueprint. Santa glanced at the screen. "Not bad. But I will need this extended later. Keep it modular, bros!"

The room fell silent as the elves realized the implications. This was just the beginning of Santa’s unhinged data modeling spree.

Your Task
The elves need your help to finish the Kid struct.

Here is what you need to do:

Add two variants to the Niceness enum: Nice and Naughty. Nice takes the number of good deeds.
Add two fields to the Kid struct: name of type String and niceness of type Niceness.
Move the is_nice function we created on Day 3 to an associated function of the Kid struct.
Finally, implement the new() associated function for the Kid struct.

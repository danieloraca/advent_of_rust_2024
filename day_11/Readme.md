Day 11: Santa's Snowy Algorithm

"LISTEN UP, YOU MAGNIFICENT, STRESSED-OUT CODERS!" Santa bellowed. "Tomorrow, we test the sleigh again. And this time, it WILL work. No more Florida nonsense. Palm trees are NOT part of the Christmas aesthetic."

He pointed dramatically at Bernard and Pepper. “You two are coming with me. If the sleigh glitches mid-flight again, I want live debugging happening in real time. No excuses.”

The elves whispered nervously. Everyone still remembered the "Florida Incident"—last week’s failed test landing in a snowless golf course. Santa didn’t appreciate the HOA complaints.

“This time,” Santa continued, pacing, “Snowball compiled every scrap of data we need for each location. Top-notch metrics. But metrics mean nothing without an algorithm. Your job is to write a function to find the snowball-densest landing spot.”

Your mission
Snowball has provided you with a Vec<Location>, now the other elves need to write a function to find the most dense area with snow.

Here is what you need to to:

Write a new() associated function for the Location struct that takes x: f64, y: f64, z: f64, area: f64, and snow.
The snow parameter must be able to accept all SnowKg, SnowLb and Snowball types.
Implement a method for the Location struct named density() that gets the density of snow in the location.
Finish the find_best_location function which takes a Vec<Location> and returns a Result<Location, Box<dyn Error>>.
That's it! 🎅

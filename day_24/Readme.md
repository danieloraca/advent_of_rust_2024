Day 24: The 64-Core Sleigh Hustle

“Alright, listen up! The sleigh’s armed with the ElfiX 9000—64 blazing-fast cores. But without a working task queue, we’re going nowhere. I need ideas, and I need them fast!”

Blitzen stretched lazily, balancing a candy cane on his nose. “What’s the big deal? Just make a list, throw everything on there, and let the sleigh do its thing.”

Bernard, the Head Elf, sighed audibly. “Blitzen, a list? Really? We need something efficient, predictable, and thread-safe. We’re not juggling reindeer treats here!”

Santa grunted. “Bernard’s right. This isn’t reindeer games. We need ultimate thread safety and dynamic dispatch. Bernard, go on.”

Bernard adjusted his glasses. “We use a VecDeque. Tasks get pushed to the back and popped from the front. It’s simple and linear.”

Blitzen tilted his head. “And how do we stop the sleigh from pulling a Rudolph and wandering off the rails?”

Bernard rolled his eyes. “Thread safety. Wrap the VecDeque in a Mutex, so each core pops tasks one at a time without stepping on each other’s hooves.”

Santa stroked his beard. “And the tasks? We’ve got a mix: deliveries, routing, even…” he checked the list, “…this goat thing.”

Bernard cut him off. “We implement a SleighTask trait. Each task adheres to it, and the queue just holds Box<dyn SleighTask>. The cores process whatever’s next without worrying about specifics.”

Blitzen yawned. “Sounds overly complicated to me.”

“Spoken like someone who’s never touched a Mutex,” Bernard snapped.

Santa chuckled. “Enough yapping! The VecDeque is our solution. Let’s build it. Christmas flies on this queue!”

The workshop buzzed with determination. With the VecDeque at its heart, the sleigh would soon conquer the skies—no reindeer complaints included.

Your Mission
The SantaSleighQueue should have a field records, make sure it is thread safe and can be mutated by multiple threads

The records list should accept either of the two task types: ElfTask and ReindeerTask

Both task types should implement the SleighTask trait

The SantaSleighQueue should have these methods:

new() -> Self: Creates a new SantaSleighQueue
enqueue adds a task to the back of the queue, returns ()
get_task pops the next task from the front of the queue, returns Option<T>
The ElfTask should have these fields:

name: String
urgency: u32
The ElfTask should have a new() associated function that creates a new ElfTask

The ReindeerTask should have these fields:

name: String
weight: String
The ReindeerTask should have a new() associated function that creates a new ReindeerTask

Don't worry about the use of the urgency and weight fields, they are just there for demonstration purposes

You can use unwrap() on the mutex lock result; for this challenge, you don't need to worry about poisoning.

Make sure all important values are pub so they can be accessed from outside the module

Make sure you have a look at the bottom of the code to see how Santa wants to use the SantaSleighQueue API.

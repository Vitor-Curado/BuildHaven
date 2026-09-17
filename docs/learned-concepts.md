# Learned concepts

## D.R.Y. K.I.S.S.E.R.

Those refer to two separate concepts which I cheekily merged, but I like to think `programmers are dry kissers`. `D.R.Y.` stands for `Do not repeat yourself`, while `K.I.S.S.` stands for `Keep It Stupidly Simple`. There is some overlap of the two; not repeating yourself usually simplifies things and vice versa, but what I learned from `dry kissing` is that it broke my initial, naive, innocent notion that I should do everything and have as few dependencies as possible, and outsourcing to X technologies is often the way to go. Just outsource that, Jimmothy, you don't need to recreate Tokio from scratch (laughs).

Over time, it stacks. It's the same difference comparing two people aged 45 of similar characteristics, but one who started using sunscreen at 18 and the other one who didn't. The codebase without outsourcing is probably so bloated and 18x with more volume to do the same thing, the complexity must be off the charts, the fellas become not just the maintainers of the application, but also of the building tools for it.

## Leg work and tradition

A software engineer is built on leg work. Yes, I learned all these concepts; the big O, the usual fellas (background jobs, migrations, cloud stuff) but the thick of the SWD/SWE wisdom comes from shadowing seniors. Comes from one coming to the meeting and seeing a senior nod, or grumpily refuse something "We do not need to write this in C++. I am writing this in TypeScript." At its finest, it's **monkey see, monkey do** turned into a job. 

Without the wisdom of a senior, one SWE/SWD matures only in the struggle, in the tension, when posed against something and having to boil with the famous "How the hell do I do this?!" This is a senior SWE/SWD; someone battered by life, by multiple different circumstances who eventually developed a taste, a north star on how to do things. If one has no access to seniors, then one must expose themselves to challenges, whimpering and crying is allowed, and... survive. 

## The building blocks / The usual suspects

These are things I learned that aren't language specific

### Big O notation (or Big O complexity).

They describe how an algorithm’s time or space requirements grow as the input size n grows.

* O(1) is Constant time: think a database lookup by ID
* O(log n) is logarithmic: basically a B-tree or a dabase index
* O(n) is linear: the frontend rendering a list they got from the backend 
* O(n log n) is linearithmic: like sorting data
* O(n^2) is quadratic: if you see this, you probably fucked up but it's quite common
* O(2^n) is exponential: if you managed to pull one of this, the boss will probably want to have a word with you.

It becomes like a checklist. No, I am not actively thinking about them, but it's good to have them in mind and if someone says to me "yo, the X function is O(n log n) when it should be O(n). Fix it, yo" I won't shit my pants [too much]. I hope no one in the office walks around saying stuff like that.

### Migrations

A migration is basically a versioned change to your database's structure. Like Git commits, but for your database schema. It's essential because over time one ands/removes rows, changes type, and it's essential for ACID databases. Technically this is specific to ACID, but... Eh, whatever, you got me. Mentioned either way. No, I have no idea about NoSQL DBs, nor do I care.

### Object storage

Fresh out of college, I thought storing stuff on a DB is okay. I learned about good ol' S3 when I was asking where do I put stuff on postgres and ChatGPT firmly told me to put it on an object storage, a "bucket". Every cloud provider has it, it's basically a linux machine hooked to a huge HDD that one rents for dirt, dirt cheap, reeeaaally cheap. 

### Background jobs

For small scale projects, these are largely optional and even unwanted as they increase complexity, maximum I can think of is refreshing this or that, but really it's multi-threading but for websites, at least that's my impression.

### The cloud

The name really fails the commoner, the lay person because I thought it was something hard. No, it isn't. The cloud is what we call managed... stuff for rent. Could be just storage, for example AWS's S3, I am not a customer but I guess one can probably just rent managed GPUs. The most famous and most straightforward one I can think off the top of my head is AWS EC2, which is a computer, headless, that you can connect from anywhere in the world. AWS is generally less accessible to the common person, and it can feel overwhelming because it's the real deal; it's serious. Comes with a lot of customisations. If one wishes for something simpler, other providers for example DigitalOcean provide droplets, which was my gateway drug into remote servers and it served me wonders, when I created an AWS account I didn't even feel overwhelmed.

Other cloud stuff I can think of, from Amazon of course but every providers should have their own, include:

* Managed DBs: Amazon has a regular ACID DB that is managed, then they have a high performance ACID DB, also managed, then the famous DynamoDB which is just a managed NoSQL DB.
* Application Load Balancers: that's when you have big boy applications and a lot of clients, and simply running one huge EC2 instance doesn't work anymore, and you have to scale horizontally (multiple EC2 instances)

It goes without saying, but storing things in a particular EC2 instance isn't a very good idea.  

### Object Relational Managers, ORMs

ORMs are a must when it comes to day-to-day software development. One learns their importance when one attempts to live without them, at first. Using a pure SQL library, for example Rust's SQLx (#2) is the equivalent of running with weights on your heels. One may get used to, and it makes one harder, but the moment they take those weights off they blast way faster. Using an ORM feels like running after one takes the weights off the heels. I kinda do think using raw SQL first is a good idea to make one thankful for an ORM. It won't restrict freedom, but the ergonomy it provides to the developer team is totally worth it, specially if there's barely any overhead. Even with overhead and performance loses, it's still worth it.

### Documentation and testing 

Documentation and testing (unit, integration) really are the 25-hydroxy-cholecalciferol and the triglycerides of a codebase. They're health markers. One can stay a couple of months on high Triglycerides and low blood vitamin D, but overtime the codebase feels sluggish, productivity gets down. One can try launching features, but they don't work, and it gets worse the longer one doesn't address it, the more moving parts the codebase has.

Please, fix your codebase's HDL, TG and 25-hydroxy-cholecalciferol. Give your codebase vitamin D pills, make it do plenty of benchmarking and stop feeding it data straight from the frontend

## Footnotes

1. Rough equivalents in other programming languages: Go's Sqlc, TS's Kysely or Drizzle, Kotlin's Exposed, Java's Jdbi, C#'s Dapper, Python's SQLAlchemy Core, C++'s SOCI

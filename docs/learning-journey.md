# My experience programming BuildHaven

This project is, fundamentally, a lot of `leg work`. It made me change my mind countless times, face concepts I didn't come close in college, it boosted my confidence in someways, but also increased my impostor syndrome in others. I had periods of frenzy adding features for the sake of it, tokenism, then periods of retracting, but I never truly gave up. 

Development has been slow because... I have been enamored more with the frenzy of implementing for the sake of implementing, blitzing, exploring software concepts and using AI to achieve things that I realised I built myself a ton of crap I don't use, and my codebase is in need of a cleanup, testing, not more features.

## Key events

* Originally in Python, Flask, gunicorn. The cloud server was a mere 0.5 GB RAM and 1 vCPU. Had to be re-written to a language known in low resource environments particularly because gunicorn was too RAM intensive for my budget; I couldn't launch the prototype into prod because of it. Rust was chosen because to me it felt new, and the claims that the language offers comparable C-levels of performance, this security, memory safety it so claims attracted me. I was led to Rust, Axum and Askama initially.
* The original idea blossomed out of a fascination and obsession with speed and performance from `min-max culture`, thus leading to make it as speedy as possible at the cost of development ergonomy and a multitude of other things
* Finally shipping something I can show to my family, friends and the world was exhilarating; I entered a tokenism frenzy to add technologies out of naivety, youthful energy, curiosity, and desperation to be able to say I have experience with X technologies on my resume in the hopes it would lead me to employement. Adding a new technology would give me a spike of dopamine, and I would be looking for the next dopamine hit, by extension, the next convoluted technology which didn't make sense. It in fact didn't. I started adding background jobs to a server with no DAUs that basically my dad visits every month or so.
* The experience gradually developed weight, shape, texture to the decisions I was making. Weeks of leg work made me understand what fundamentally this is, just a web app, and lose a portion of the fear and impostor syndrome around it. I lost part of the naivety that comes with being a junior, I felt framework anxiety, learned when something doesn't need to be written in X, Y or Z, learned to say less "yes" and more "no", learned to focus on the scope at hand to deliver a result, learned some things are the way they are for a reason, for example the importance of tests (unit, integration and E2E), the project having more than one master/main branch
* Learned a grand-view of what a web-app is, learned how languages can affect performance
* Due to the project always being Linux intensive, learned how to deal with SELinux policies and generally speaking how to get around the terminal without wanting to crash my head against the keyboard

## How it started

I started this project first in Python. I started because I graduated college realising I hadn't done anything of my own; having a website of my own seemed pretty logical. Do something simple In the process of trying to ship it, and inevitably having to rewrite it in Rust, I started thinking I would program things from scratch, as few dependencies as possible. Slowly, over the course of weeks to months, I came in contact that there's always a dependency, and with control comes responsibility. 

I remember to this day how it started, and it was with the thought "Fuck it, I haven't programmed anything meaningful during my program, let me ask ChatGPT what are the leanest, fastest tech stack that I could program with." At the time, I remember to be obsessed with raw speed, and I also had no idea how the speed of each and every part mattered. I remember the stack it gave me was:

(Everything Rust, because I explicitly asked for it)
* Axum: 
* Askama (templating engine)
* SQLx ()
* Tokio (OFC, async runtime)

That's all that I knew. Everything was so bright; I remember not knowing what SSR was, nor CI/CD. These things were as alien to me as fossils in a museum. I remember

## Cherishing those little moments

You had no idea how happy, with a wide smile, I was with learning I can do little things such as customising the favicon per endpoint, or the page title. Running my first Dockerfile and building my first image, running my first `podman-compose up` instead of using `cargo run`

## The feeling it's all crumbling

As the codebase actually increases in total size and there are multiple moving parts, I started understanding why this is a job. Change one thing, and 33 other parts that were connected to it either break or become janky. Everything feels like it constantly needs attention, and adding one feature, which feels so good, if rushed can really be a poor decision that one time may be okay, but like smocking cigarettes or eating fast food, it stacks over time.

As the `developer's high` fades, the problems associated with a sluggish codebase are akin to `hangover` and they gradually arise, whether it be days, weeks or months. The `feature rush` is then replaced by the `developer's guilt` by not being a good dev and implementing unit, integration tests (and so forth) and not checking in with other parts of the codebase.

## Compensation

Culture often says that `men like to compensate` by owning big cars. That's the most stereotypical example. I think the girlies too experience the same phenomemnon, I don't think at all it is reserved by guys to compensate, it's just girls express compensation behaviour in a different way. Anyway, the point is... we, developers (gender agnostic), like to compensate too, in our own ways. Some of us like, are fascinated by efficiency, others by speed, some by how simple the code is, others how complex the code is 

## College

### Things I realise college did not teach me that I think should have

#### Containerisation 

By far, it's one of those things I look back and think: "Why didn't they include this?" It's a file, there's three steps, it's logical, there's no licensing, no need for external companies like the case for an EC2 instance to get them cloud knowledge, and the reward is so high. 

I would make a case:

* Students could have Dockerfiles as early as on the second semester
* They could have in their projects `compose.yml` files

And by the time they graduate, containerisation is already second nature, normal.

#### Git stuff and documentation

What are code reviews? Pull requests? "Yo, what does foo(x) does? Does anyone remember?" (#1) Both documentation and Git lingo is something that could come more naturally in the context of group projects, where the need for documentation can arise. How they would grade this, I have no clue, but it's such a fundamental thing that I felt wasn't addressed. I think both should not be the topic of a class, rather it should be something all classes should enforce or foster, it should be an item on the rubric not "Advanced Git class" or "Advanced documentation class: how to gaslight the tech leads"

#### Language agnostic concepts

Yes, I had design patterns class. What I didn't have was a class to learn what Dependency Injection is. It matters little, but I think it would broaden the college graduate that languages have interchangeable parts, while having unique or specific use cases. 

### Things college won't/can't prepare you for or mine didn't

#### Tradeoffs 

"Should we write this in X, Y, Z language/framework, or A, B, C? Should we go monolith for this, or that? Where can we afford I, J, Ts?" 

College classes are not made for this, I felt. I had one, the capstone project, where we had full autorisation to move from point A to point B. Classes are made to, for example, use this one technology, such as Java Hibernate or Java Server Faces and sure we do encounter challenges first implementing the technology and the actual problem contained in the assignnment, not... "We have this problem, how are we going to fix it?" Barely did I have to chance for that. I never had to think we need a NoSQL DB because the way the team is going using a DynamoDB over AWS RDS would be a better overall fit, that scene never happened and I think college isn't structured to be able to offer that; this is advanced Java class and we're teaching you JSF, which is what happened to me.

#### Discipline 

"When are we going to add PgBouncer to..." No, Jimmothy, we won't. It's a college project, and only the professor is gonna look at it, once. Once, Jimmothy. Never did I have this experience where we want to implement things and self-restrain or we are chastised not to do it, or strongly encouraged not to 

#### The urge to build 

If anything, college clamps down on your desire to see, to witness, to do. Part of the reason why I took a gap year after graduating college was because making software didn't feel rewarding. After about a year, making stuff felt good again.

### Things college taught me that are useful

* **DBA stuff:** This is an addendum because I have migrated from Windows to Fedora Linux and installing databases, for example, Postgres is much, much easier if you're okay not using a GUI, and had I not had that DBA class I would have been shitting my pants to use pgsql, which surprisingly I wasn't. 
* **Politiking:** At the end of the day, we're people working towards a common goal. I wasn't "taught" per se, but I was in a lot of group projects and college was able not to set the expectations for a real workplace because we know there's a lot of people who don't do their work, but because of simple exposure. Getting accustomed to work in a group setting is actually useful.
* **The fundamentals:** Web stuff (static HTML, CSS and JS web pages), Server Side Rendering (SSR), ORMs, Object Oriented Programming (OOP). 

## Trying new technologies

I remember trying this Rust CSR technology, leptos, and yes it was cool but I remember feeling betrayed by it. It promised to solve a job I was itching to solve, CSR, using Rust and not JavaScript/TypeScript, and it promised to be faster because of Web Assembly. Yes, it was technically right and it was technically faster, not safer, but boy did I discover `developer comfort` was a thing, and it wasn't what I was looking for. I was thinking, naively, just because it's a Rust library that it would eliminate CSP warnings. It didn't. Trying to implement that was one week in total; trying it, then eventually giving it up and I remember returning to 100% SSR 

## Things I was wrong about (that I learned along the way)

Developing BuildHaven changed me. Starting, my head was sort of thinking: 

* Performance was the primary engineering concern
* Fewer dependencies automatically meant better software
* Using more sophisticated technology demonstrated competence
* Production software should use the most technically impressive architecture I could build
* Rewriting something in Rust automatically made it better
* Shipping meant reaching the end of development
* Knowing a technology meant being able to put it on my résumé
* Adding features was progress

## Things I now think

* Don't demonise JavaScript, nor any programming language, nor any technology for that matter. I got a bit of beef with JavaScript because of Content Security Policy warnings and because of the negative backlash of companies writing desktop apps with JavaScript, but the problem never was JavaScript, it was the companies writing desktop apps with it. CSP warnings are fixable, and `one learns to stop worrying about the wrongdoings of others, much less to place guilt over innocents`

I still try to not use <div> though. I do, I try.

## Footnotes

1. Victor, 6:00 PM, with his mates in the library, two days before an assignment submission.